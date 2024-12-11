#[derive(Clone, Debug)]
struct Block {
    id: usize,
    size: usize,
    original_size: usize,
    free_blocks: Vec<usize>,
    free_blocks_left: usize,
}

pub fn solve_part_two(input: &str) -> usize {
    let input_chars: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut files: Vec<Block> = input_chars
        .windows(2)
        .step_by(2)
        .enumerate()
        .map(|(id, cs)| {
            return Block {
                id,
                size: cs[0],
                original_size: cs[0],
                free_blocks: Vec::new(),
                free_blocks_left: cs[1],
            };
        })
        .collect();
    files.push(Block {
        id: files[files.len() - 1].id + 1,
        size: input_chars[input_chars.len() - 1],
        original_size: input_chars[input_chars.len() - 1],
        free_blocks: Vec::new(),
        free_blocks_left: 0,
    });

    compact_files(&mut files);
    calculate_checksum(&files)
}

fn compact_files(files: &mut Vec<Block>) {
    let mut current_block = files.len() - 1;
    loop {
        if current_block == 0 {
            break;
        }

        let size_needed = files[current_block].size;
        let id = files[current_block].id;
        for i in 0..current_block {
            if files[i].free_blocks_left < size_needed {
                continue;
            }

            files[current_block].size = 0;
            for _ in 0..size_needed {
                files[i].free_blocks.push(id);
            }

            files[i].free_blocks_left -= size_needed;
            break;
        }

        current_block -= 1;
    }
}

fn calculate_checksum(files: &Vec<Block>) -> usize {
    let mut current_block = 0;
    let mut current_file_index = 0;
    let mut checksum = 0;
    loop {
        if current_block == files.len() {
            break;
        }

        if files[current_block].size == 0 {
            current_file_index += files[current_block].original_size;
        }

        for _ in 0..files[current_block].size {
            checksum += files[current_block].id * current_file_index;
            current_file_index += 1;
        }

        for n in &files[current_block].free_blocks {
            checksum += n * current_file_index;
            current_file_index += 1;
        }

        current_file_index += files[current_block].free_blocks_left;
        current_block += 1;
    }

    checksum
}
