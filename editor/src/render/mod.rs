// Copyright 2018 Google LLC, licensed under http://www.apache.org/licenses/LICENSE-2.0

mod area;
mod building;
mod bus_stop;
mod car;
mod extra_shape;
mod intersection;
mod lane;
mod map;
mod parcel;
mod pedestrian;
mod turn;

use ezgui::{Color, GfxCtx};
use geom::{Bounds, Pt2D};
use map_model::Map;
use objects::{Ctx, ID};
pub use render::area::DrawArea;
pub use render::car::DrawCar;
pub use render::lane::DrawLane;
pub use render::map::DrawMap;
pub use render::pedestrian::DrawPedestrian;
pub use render::turn::DrawTurn;
use std::f64;

// These are all in meters
const PARCEL_BOUNDARY_THICKNESS: f64 = 0.5;
const EXTRA_SHAPE_THICKNESS: f64 = 1.0;
const EXTRA_SHAPE_POINT_RADIUS: f64 = 1.0;

pub const BIG_ARROW_THICKNESS: f64 = 0.5;
const TURN_ICON_ARROW_THICKNESS: f64 = BIG_ARROW_THICKNESS / 3.0;
const BIG_ARROW_TIP_LENGTH: f64 = 1.0;
const TURN_ICON_ARROW_TIP_LENGTH: f64 = BIG_ARROW_TIP_LENGTH * 0.8;
const TURN_ICON_ARROW_LENGTH: f64 = 2.0;

pub trait Renderable {
    fn get_id(&self) -> ID;
    fn draw(&self, g: &mut GfxCtx, opts: RenderOptions, ctx: Ctx);
    fn get_bounds(&self) -> Bounds;
    fn contains_pt(&self, pt: Pt2D) -> bool;
    fn tooltip_lines(&self, map: &Map) -> Vec<String>;
}

pub struct RenderOptions {
    // The "main" color for the object, if available.
    pub color: Option<Color>,
    pub cam_zoom: f64,
    pub debug_mode: bool,
}
