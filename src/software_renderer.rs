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
) -> Option<(f32, f32)> {
    let size = paint_ctx.size();
    let (width, height) = (size.width as u32, size.height as u32);
    let dimensions = renderer.image().dimensions();

    let new_dims = renderer.render(state, [width, height]);

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
