/// https://towardsdatascience.com/interview-question-select-a-random-line-from-a-file-in-rust-c0a8cddcddfb

use std::io::{Result as STDResult};
use tokio::{fs::File, io::{BufReader, Lines}};

async fn try_nth(lines: &mut Lines<BufReader<File>>, n: usize) -> STDResult<Option<String>> {
    for _ in 0..n {
        if lines.next_line().await?.is_none() {
            return Ok(None);
        }
    }
    lines.next_line().await
}

pub async fn one_pass_skip(mut iter: Lines<BufReader<File>>, r: f64) -> STDResult<Option<String>> {
    let mut offset = 1;
    let mut index1 = 1;
    let mut random_item = None;
    while let Some(item) = try_nth(&mut iter, offset - 1).await? {
        random_item = Some(item);
        offset = ((r * (index1 as f64) / (1.0 - r)).ceil() as usize).max(1);
        index1 += offset;
    }

    Ok(random_item)
}
