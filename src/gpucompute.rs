use anyhow::Result;
use fastrand;
use flume::bounded;
use pollster;
use rayon::prelude::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub fn run_on_gpu(gen_quant: usize, bar_incrementer: impl Fn(usize) + Send + Sync) -> Result<usize> {
    let res = pollster::block_on(do_computation(gen_quant, bar_incrementer))?;
    Ok(res)
}

async fn do_computation(gen_quant: usize, bar_incrementer: impl Fn(usize) + Send + Sync) -> Result<usize> {
    let send_quant: usize = 4_000_000;
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter.request_device(&Default::default()).await.unwrap();

    let shader = device.create_shader_module(wgpu::include_wgsl!("introduction.wgsl"));

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Introduction Compute Pipeline"),
        layout: None,
        module: &shader,
        entry_point: None,
        compilation_options: Default::default(),
        cache: Default::default(),
    });

    let mut master_max_roll = 0u32;

    for _ in 0..(gen_quant / send_quant) {
        let rendered_rands: Vec<u32> = parallel_gen_u32(send_quant);
        let send_slice = &rendered_rands[0..];

        let input_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("input"),
            contents: bytemuck::cast_slice(send_slice),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("output"),
            size: input_buffer.size(),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let temp_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("temp"),
            size: input_buffer.size(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&Default::default());

        let send_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("input"),
            contents: bytemuck::cast_slice(send_slice),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
        });

        {
            // We specified 64 threads per workgroup in the shader, so we need to compute how many
            // workgroups we need to dispatch.
            let num_dispatches = send_quant.div_ceil(256) as u32;

            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(num_dispatches, 1, 1);
        }

        encoder.copy_buffer_to_buffer(&send_buffer, 0, &input_buffer, 0, input_buffer.size());
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &temp_buffer, 0, output_buffer.size());
        queue.submit([encoder.finish()]);

        {
            // The mapping process is async, so we'll need to create a channel to get
            // the success flag for our mapping
            let (tx, rx) = bounded(1);

            // We send the success or failure of our mapping via a callback
            temp_buffer.map_async(wgpu::MapMode::Read, .., move |result| {
                tx.send(result).unwrap()
            });

            // The callback we submitted to map async will only get called after the
            // device is polled or the queue submitted
            device.poll(wgpu::PollType::wait_indefinitely())?;

            // We check if the mapping was successful here
            rx.recv_async().await??;

            // We then get the bytes that were stored in the buffer
            let output_data = temp_buffer.get_mapped_range(..);

            let newresults = bytemuck::cast_slice::<u8, u32>(&output_data);
            let local_max_roll = *newresults.into_iter().max().unwrap();
            //println!("local max for run has been {local_max_roll}");
            if local_max_roll > master_max_roll {
                master_max_roll = local_max_roll;
            }
        }
        temp_buffer.unmap();
        bar_incrementer(send_quant)
    }
    // We need to unmap the buffer to be able to use it again

    Ok(master_max_roll as usize)
}

fn parallel_gen_u32(rounds: usize) -> Vec<u32> {
    (0..rounds)
        .into_par_iter()
        .map(|_| fastrand::u32(0..u32::MAX))
        .collect()
}
