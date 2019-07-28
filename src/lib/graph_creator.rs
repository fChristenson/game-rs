use amethyst::ecs::{ReadExpect, Resources, SystemData};
use amethyst::renderer::pass::DrawFlat2DDesc;
use amethyst::renderer::rendy::graph::present::PresentNode;
use amethyst::renderer::rendy::hal::command::{ClearDepthStencil, ClearValue};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::{
  Factory, Format, GraphBuilder, GraphCreator, Kind, RenderGroupDesc, SubpassBuilder,
};
use amethyst::window::{ScreenDimensions, Window};
use std::ops::Deref;

#[derive(Default)]
pub struct GameGraphCreator {
  dimensions: Option<ScreenDimensions>,
  dirty: bool,
}

impl GraphCreator<DefaultBackend> for GameGraphCreator {
  fn rebuild(&mut self, res: &Resources) -> bool {
    let new_dimensions = res.try_fetch::<ScreenDimensions>();

    if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
      self.dirty = true;
      self.dimensions = new_dimensions.map(|d| d.clone());
      return false;
    }

    self.dirty
  }

  fn builder(
    &mut self,
    factory: &mut Factory<DefaultBackend>,
    res: &Resources,
  ) -> GraphBuilder<DefaultBackend, Resources> {
    self.dirty = false;

    // Retrieve a reference to the target window, which is created by the WindowBundle
    let window = <ReadExpect<'_, Window>>::fetch(res);
    let dimensions = self.dimensions.as_ref().unwrap();
    let window_kind = Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

    // Create a new drawing surface in our window
    let surface = factory.create_surface(&window);
    let surface_format = factory.get_surface_format(&surface);

    // Begin building our RenderGraph
    let mut graph_builder = GraphBuilder::new();
    let color = graph_builder.create_image(
      window_kind,
      1,
      surface_format,
      Some(ClearValue::Color([0.529, 0.808, 0.98, 1.0].into())),
    );

    let depth = graph_builder.create_image(
      window_kind,
      1,
      Format::D32Sfloat,
      Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
    );

    // Create our first `Subpass`, which contains the DrawFlat2D render group.
    // We pass the subpass builder a description of our groups for construction
    let pass = graph_builder.add_node(
      SubpassBuilder::new()
        .with_group(DrawFlat2DDesc::default().builder()) // Draws sprites
        .with_color(color)
        .with_depth_stencil(depth)
        .into_pass(),
    );

    // Finally, add the pass to the graph
    let _present =
      graph_builder.add_node(PresentNode::builder(factory, surface, color).with_dependency(pass));

    graph_builder
  }
}
