use bevy::prelude::*;

pub struct MenuMaterials {
  pub root: Handle<ColorMaterial>,
  pub border: Handle<ColorMaterial>,
  pub menu: Handle<ColorMaterial>,
  pub button: Handle<ColorMaterial>,
  pub button_hovered: Handle<ColorMaterial>,
  pub button_pressed: Handle<ColorMaterial>,
  pub button_text: Color,
}

impl FromWorld for MenuMaterials {
  fn from_world(world: &mut World) -> Self {
      let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
      MenuMaterials {
          root: materials.add(Color::NONE.into()),
          border: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
          menu: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
          button: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
          button_hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
          button_pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
          button_text: Color::WHITE,
      }
  }
}

pub fn button_system(
  materials: Res<MenuMaterials>,
  mut buttons: Query<
      (&Interaction, &mut Handle<ColorMaterial>),
      (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut material) in buttons.iter_mut() {
      match *interaction {
          Interaction::Clicked => *material = materials.button_pressed.clone(),
          Interaction::Hovered => *material = materials.button_hovered.clone(),
          Interaction::None => *material = materials.button.clone(),
      }
  }
}

pub fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
  NodeBundle {
      style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          flex_direction: FlexDirection::ColumnReverse,
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..Default::default()
      },
      material: materials.root.clone(),
      ..Default::default()
  }
}

pub fn border(materials: &Res<MenuMaterials>) -> NodeBundle {
  NodeBundle {
      style: Style {
          size: Size::new(Val::Px(400.0), Val::Auto),
          border: Rect::all(Val::Px(8.0)),
          ..Default::default()
      },
      material: materials.border.clone(),
      ..Default::default()
  }
}

pub fn menu_background(materials: &Res<MenuMaterials>) -> NodeBundle {
  NodeBundle {
      style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          flex_direction: FlexDirection::ColumnReverse,
          padding: Rect::all(Val::Px(5.0)),
          ..Default::default()
      },
      material: materials.menu.clone(),
      ..Default::default()
  }
}

pub fn button(materials: &Res<MenuMaterials>) -> ButtonBundle {
  ButtonBundle {
      style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..Default::default()
      },
      material: materials.button.clone(),
      ..Default::default()
  }
}

pub fn button_text(
  asset_server: &Res<AssetServer>,
  materials: &Res<MenuMaterials>,
  label: &str,
) -> TextBundle {
  return TextBundle {
      style: Style {
          margin: Rect::all(Val::Px(10.0)),
          ..Default::default()
      },
      text: Text::with_section(
          label,
          TextStyle {
              font: asset_server.load("fonts/FiraSans-Bold.ttf"),
              font_size: 30.0,
              color: materials.button_text.clone(),
          },
          Default::default(),
      ),
      ..Default::default()
  };
}