// A read-only storage buffer that stores and array of unsigned 32bit integers
@group(0) @binding(0) var<storage, read> input: array<u32>;
// This storage buffer can be read from and written to
@group(0) @binding(1) var<storage, read_write> output: array<u32>;

// Tells wgpu that this function is a valid compute pipeline entry_point
@compute
// Specifies the "dimension" of this work group
@workgroup_size(256)
fn main(
    // global_invocation_id specifies our position in the invocation grid
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>
) {
    let index = global_invocation_id.x;
    let total = arrayLength(&input);

    // workgroup_size may not be a multiple of the array size so
    // we need to exit out a thread that would index out of bounds.
    if (index >= total) {
        return;
    }

    let seed1 = input[global_invocation_id.y];
    let seed2 = select(input[index + 1u], input[0], (index < (total - 1u)));
    
    let result = do_simulation(seed1, seed2);

    output[global_invocation_id.x] = result;
}

fn hash_u32(x_in: u32) -> u32 {
    var x = x_in;
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    x += (x << 15u);
    return x;
}

fn do_simulation(seed1: u32, seed2: u32) -> u32 {

    // We get 16 d4 rolls per u32, and need 231 rolls. 231 / 16 = 14 rem 7, and we get
    // two random u32 per iteration, so we need to get our 7 values and then iterate
    // 7 times
    var value1 = seed1;
    var value2 = seed2;
    var accum = get_results_from_two_32s(value1, value2, 7u);
    for (var i = 0u; i < 7u; i++) {
        value1 = hash_u32(value1);
        value2 = hash_u32(value2);
        accum += get_results_from_two_32s(value1, value2, 32u);
        //accum += get_results_from_u32(value1, 16u);
        //accum += get_results_from_u32(value2, 16u);
    }
    return accum;
}

fn get_results_from_two_32s(value1: u32, value2: u32, pairs_to_get: u32) -> u32 {
    //todo- bitwise & the two values together, and get the pairs by seeing the last bit
    // after shifting
    let newval = value1 & value2;
    var greater = 0u;
    for (var i = 0u; i < pairs_to_get; i++) {
        if ((newval >> i) & 1u) == 1u {
            greater++;
        }
    }
    return greater;
}
