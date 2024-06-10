use std::{cell::RefCell, rc::Rc};

use druid::{
    kurbo::PathEl,
    piet::{Device, ImageFormat, InterpolationMode, PaintBrush, PietImage},
    Affine, Color, ImageBuf, LinearGradient, PaintCtx, Point, Rect, RenderContext, UnitPoint,
};
use livesplit_core::{layout::LayoutState, rendering::software::Renderer};

pub fn render_scene(
    paint_ctx: &mut PaintCtx,
    bottom_image: &mut Option<PietImage>,
    renderer: &mut Renderer,
    state: &LayoutState,
) -> Option<(f64, f64)> {
    let size = paint_ctx.size();
    let scale = paint_ctx.scale();
    let scaled_width = size.width * scale.x();
    let scaled_height = size.height * scale.y();

    let (width, height) = (scaled_width as u32, scaled_height as u32);
    let dimensions = renderer.image().dimensions();

    let new_scaled_dims = renderer.render(state, image_cache, [width, height]);
    let new_dims = new_scaled_dims.map(|[w, h]| scale.px_to_dp_xy(w, h));

    let bottom_image = if bottom_image.is_none() || dimensions != (width, height) {
        bottom_image.insert(
            paint_ctx
                .make_image(
                    width as usize,
                    height as usize,
                    renderer.image_data(),
                    ImageFormat::RgbaPremul,
                )
                .unwrap(),
        )
    } else {
        let bottom_image = bottom_image.as_mut().unwrap();
        // TODO: Update
        *bottom_image = paint_ctx
            .make_image(
                width as usize,
                height as usize,
                renderer.image_data(),
                ImageFormat::RgbaPremul,
            )
            .unwrap();
        // paint_ctx.update_image(bottom_image, renderer.image_data());
        bottom_image
    };

    paint_ctx.draw_image(
        bottom_image,
        Rect::from_origin_size(Point::ZERO, size),
        InterpolationMode::NearestNeighbor,
    );

    new_dims
}
