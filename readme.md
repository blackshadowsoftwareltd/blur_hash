# Blur Hash

## Dimensions Calculation

`>` `pixels` will be `min` and `max` will be calculated
`<` `pixels` will be `max` and `min` will be calculated

```rust
fn dimensions(w: u32, h: u32) -> (u32, u32) {
    let pixels = 50; // ? 50 pixels
 
    let result = if w > h {
        let ratio = (w as f32) / (h as f32);
        ((pixels as f32 * ratio) as u32, pixels)
    } else {
        let ratio = (h as f32) / (w as f32);
        (pixels, (pixels as f32 * ratio) as u32)
    };
    result
}
```
