//! Offline encoding experiment, never evidence of terminal protocol support.
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Size},
    widgets::Widget,
};
use ratatui_image::{
    picker::{Picker, ProtocolType},
    Image, Resize,
};
use serde_json::json;
use std::{fs, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::env::args()
        .nth(1)
        .ok_or("expected authorized table PNG")?;
    let bitmap = image::open(&source)?;
    let mut rows = Vec::new();
    for kind in [
        ProtocolType::Halfblocks,
        ProtocolType::Sixel,
        ProtocolType::Kitty,
        ProtocolType::Iterm2,
    ] {
        let mut picker = Picker::halfblocks();
        picker.set_protocol_type(kind);
        for (width, height) in [(80, 30), (56, 40), (120, 40)] {
            let started = Instant::now();
            let protocol =
                picker.new_protocol(bitmap.clone(), Size::new(width, height), Resize::Fit(None))?;
            let encode_us = started.elapsed().as_micros();
            let area = Rect::new(0, 0, width, height);
            let mut buffer = Buffer::empty(area);
            let started = Instant::now();
            for _ in 0..100 {
                Image::new(&protocol).render(area, &mut buffer);
            }
            let redraw_us = started.elapsed().as_micros() / 100;
            let encoded_cell_bytes: usize =
                buffer.content.iter().map(|cell| cell.symbol().len()).sum();
            rows.push(json!({"protocol":format!("{kind:?}"),"width":width,"height":height,
                "encode_us":encode_us,"cached_redraw_us":redraw_us,"encoded_cell_bytes":encoded_cell_bytes,
                "source_rgba_bytes":bitmap.width()*bitmap.height()*4}));
        }
    }
    fs::write(
        "output/sprint21/bitmap-benchmark.json",
        serde_json::to_vec_pretty(&json!({
        "source":source,"font_cell":"10x20 assumed for offline encoding only",
        "terminal_support_proven":false,"samples":rows}))?,
    )?;
    println!("BITMAP_SPIKE_PASS samples={}", rows.len());
    Ok(())
}
