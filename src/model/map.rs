use std::io::Read;
use std::str::FromStr;
use std::path::Path;
use std::fs::File;

use xml::attribute::OwnedAttribute;

use error::Error;
use model::color::Color;
use model::data::Data;
use model::image::Image;
use model::property::{PropertyCollection, Properties};
use model::reader::{self, TmxReader, ElementReader};
use model::shape::Shape;
use model::tileset::{Tileset};

define_iterator_wrapper!(Tilesets, Tileset);
define_iterator_wrapper!(Layers, Layer);
define_iterator_wrapper!(ImageLayers, ImageLayer);
define_iterator_wrapper!(ObjectGroups, ObjectGroup);
define_iterator_wrapper!(Objects, Object);

#[derive(Debug, Default)]
pub struct Map {
    bg_color: Option<Color>,
    version: String,
    orientation: Orientation,
    render_order: RenderOrder,
    width: u32,
    height: u32,
    tile_width: u32,
    tile_height: u32,
    hex_side_length: Option<u32>,
    stagger_axis: Option<Axis>,
    stagger_index: Option<Index>,
    next_object_id: u32,
    tilesets: Vec<Tileset>,
    layers: Vec<Layer>,
    image_layers: Vec<ImageLayer>,
    object_groups: Vec<ObjectGroup>,
}

impl Map {
    pub fn open<P: AsRef<Path>>(path: P) -> ::Result<Map> {
        let file = try!(File::open(path));
        let mut reader = TmxReader::new(file);
        reader.read_map()
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    fn set_version<S: Into<String>>(&mut self, version: S) {
        self.version = version.into();
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    pub fn render_order(&self) -> RenderOrder {
        self.render_order
    }

    fn set_render_order(&mut self, render_order: RenderOrder) {
        self.render_order = render_order;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn tile_width(&self) -> u32 {
        self.tile_width
    }

    fn set_tile_width(&mut self, tile_width: u32) {
        self.tile_width = tile_width;
    }

    pub fn tile_height(&self) -> u32 {
        self.tile_height
    }

    fn set_tile_height(&mut self, tile_height: u32) {
        self.tile_height = tile_height;
    }

    pub fn hex_side_length(&self) -> Option<u32> {
        self.hex_side_length
    }

    fn set_hex_side_length(&mut self, hex_side_length: u32) {
        self.hex_side_length = Some(hex_side_length);
    }

    pub fn stagger_axis(&self) -> Option<Axis> {
        self.stagger_axis
    }

    fn set_stagger_axis(&mut self, stagger_axis: Axis) {
        self.stagger_axis = Some(stagger_axis);
    }

    pub fn stagger_index(&self) -> Option<Index> {
        self.stagger_index
    }

    fn set_stagger_index(&mut self, stagger_index: Index) {
        self.stagger_index = Some(stagger_index);
    }

    pub fn background_color(&self) -> Option<&Color> {
        self.bg_color.as_ref()
    }

    fn set_background_color(&mut self, color: Color) {
        self.bg_color = Some(color);
    }

    pub fn next_object_id(&self) -> u32 {
        self.next_object_id
    }

    fn set_next_object_id(&mut self, next_object_id: u32) {
        self.next_object_id = next_object_id;
    }

    pub fn tilesets(&self) -> Tilesets {
        Tilesets(self.tilesets.iter())
    }

    fn add_tileset(&mut self, tileset: Tileset) {
        self.tilesets.push(tileset);
    }

    pub fn layers(&self) -> Layers {
        Layers(self.layers.iter())
    }

    fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn image_layers(&self) -> ImageLayers {
        ImageLayers(self.image_layers.iter())
    }

    fn add_image_layer(&mut self, image_layer: ImageLayer) {
        self.image_layers.push(image_layer);
    }

    pub fn object_groups(&self) -> ObjectGroups {
        ObjectGroups(self.object_groups.iter())
    }

    fn add_object_group(&mut self, object_group: ObjectGroup) {
        self.object_groups.push(object_group);
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Map> {
        let mut tmx = TmxReader::new(s.as_bytes());
        tmx.read_map()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

impl FromStr for Axis {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Axis> {
        match s {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            _ => Err(Error::BadAxis(s.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Index {
    Even,
    Odd,
}

impl FromStr for Index {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Index> {
        match s {
            "even" => Ok(Index::Even),
            "odd" => Ok(Index::Odd),
            _ => Err(Error::BadIndex(s.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::Orthogonal
    }
}

impl FromStr for Orientation {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Orientation> {
        match s {
            "orthogonal" => Ok(Orientation::Orthogonal),
            "isometric" => Ok(Orientation::Isometric),
            "staggered" => Ok(Orientation::Staggered),
            "hexagonal" => Ok(Orientation::Hexagonal),
            _ => Err(Error::BadOrientation(s.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl Default for RenderOrder {
    fn default() -> RenderOrder {
        RenderOrder::RightDown
    }
}

impl FromStr for RenderOrder {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<RenderOrder> {
        match s {
            "right-down" => Ok(RenderOrder::RightDown),
            "right-up" => Ok(RenderOrder::RightUp),
            "left-down" => Ok(RenderOrder::LeftDown),
            "left-up" => Ok(RenderOrder::LeftUp),
            _ => Err(Error::BadRenderOrder(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct Layer {
    name: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    opacity: Opacity,
    visible: bool,
    offset_x: i32,
    offset_y: i32,
    properties: PropertyCollection,
    data: Option<Data>,
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            name: String::default(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            opacity: 1.0,
            visible: true,
            offset_x: 0,
            offset_y: 0,
            properties: PropertyCollection::new(),
            data: None,
        }
    }
}

impl Layer {
    pub fn name(&self) -> &str {
        &self.name
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    fn set_opacity(&mut self, opacity: Opacity) {
        self.opacity = opacity;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn offset_x(&self) -> i32 {
        self.offset_x
    }

    fn set_offset_x(&mut self, offset_x: i32) {
        self.offset_x = offset_x;
    }

    pub fn offset_y(&self) -> i32 {
        self.offset_y
    }

    fn set_offset_y(&mut self, offset_y: i32) {
        self.offset_y = offset_y;
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

    pub fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }

    fn set_data(&mut self, data: Data) {
        self.data = Some(data);
    }
}

#[derive(Debug)]
pub struct ImageLayer {
    name: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    opacity: Opacity,
    visible: bool,
    offset_x: i32,
    offset_y: i32,
    properties: PropertyCollection,
    image: Option<Image>,
}

impl Default for ImageLayer {
    fn default() -> ImageLayer {
        ImageLayer {
            name: String::default(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            opacity: 1.0,
            visible: true,
            offset_x: 0,
            offset_y: 0,
            properties: PropertyCollection::new(),
            image: None,
        }
    }
}

impl ImageLayer {
    pub fn name(&self) -> &str {
        &self.name
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn offset_x(&self) -> i32 {
        self.offset_x
    }

    fn set_offset_x(&mut self, offset_x: i32) {
        self.offset_x = offset_x;
    }

    pub fn offset_y(&self) -> i32 {
        self.offset_y
    }

    fn set_offset_y(&mut self, offset_y: i32) {
        self.offset_y = offset_y;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    fn set_opacity(&mut self, opacity: Opacity) {
        self.opacity = opacity;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

    pub fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }
}

pub type Opacity = f64;

#[derive(Debug)]
pub struct ObjectGroup {
    name: String,
    color: Option<Color>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    opacity: Opacity,
    visible: bool,
    offset_x: i32,
    offset_y: i32,
    draw_order: DrawOrder,
    properties: PropertyCollection,
    objects: Vec<Object>,
}

impl ObjectGroup {
    pub fn name(&self) -> &str {
        &self.name
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }

    fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    fn set_opacity(&mut self, opacity: Opacity) {
        self.opacity = opacity;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn offset_x(&self) -> i32 {
        self.offset_x
    }

    fn set_offset_x(&mut self, offset_x: i32) {
        self.offset_x = offset_x;
    }

    pub fn offset_y(&self) -> i32 {
        self.offset_y
    }

    fn set_offset_y(&mut self, offset_y: i32) {
        self.offset_y = offset_y;
    }

    pub fn draw_order(&self) -> DrawOrder {
        self.draw_order
    }

    fn set_draw_order(&mut self, draw_order: DrawOrder) {
        self.draw_order = draw_order;
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

    pub fn objects(&self) -> Objects {
        Objects(self.objects.iter())
    }

    fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

impl Default for ObjectGroup {
    fn default() -> ObjectGroup {
        ObjectGroup {
            name: String::default(),
            color: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            opacity: 1.0,
            visible: true,
            offset_x: 0,
            offset_y: 0,
            draw_order: DrawOrder::TopDown,
            properties: PropertyCollection::new(),
            objects: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Object {
    id: u32,
    name: String,
    object_type: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    rotation: f32,
    visible: bool,
    gid: Option<u32>,
    properties: PropertyCollection,
    shape: Option<Shape>,
}

impl Default for Object {
    fn default() -> Object {
        Object {
            id: 0,
            name: String::new(),
            object_type: String::new(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            rotation: 0.0,
            visible: true,
            gid: None,
            properties: PropertyCollection::new(),
            shape: None,
        }
    }
}

impl Object {
    pub fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn object_type(&self) -> &str {
        &self.object_type
    }

    fn set_object_type<S: Into<String>>(&mut self, object_type: S) {
        self.object_type = object_type.into();
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn gid(&self) -> Option<u32> {
        self.gid
    }

    fn set_gid(&mut self, gid: u32) {
        self.gid = Some(gid);
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

    pub fn shape(&self) -> Option<&Shape> {
        self.shape.as_ref()
    }

    fn set_shape<S: Into<Shape>>(&mut self, shape: S) {
        self.shape = Some(shape.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawOrder {
    TopDown,
    Index,
}

impl FromStr for DrawOrder {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<DrawOrder> {
        match s {
            "topdown" => Ok(DrawOrder::TopDown),
            "index" => Ok(DrawOrder::Index),
            _ => Err(Error::BadDrawOrder(s.to_string())),
        }
    }
}

impl<R: Read> ElementReader<Map> for TmxReader<R> {
    fn read_attributes(&mut self, map: &mut Map, name: &str, value: &str) -> ::Result<()> {
        match name {
            "version" => {
                map.set_version(value);
            }
            "orientation" => {
                let orientation = try!(Orientation::from_str(value));
                map.set_orientation(orientation);
            }
            "renderorder" => {
                let render_order = try!(RenderOrder::from_str(value));
                map.set_render_order(render_order);
            }
            "width" => {
                let width = try!(reader::read_num(value));
                map.set_width(width);
            }
            "height" => {
                let height = try!(reader::read_num(value));
                map.set_height(height);
            }
            "tilewidth" => {
                let tile_width = try!(reader::read_num(value));
                map.set_tile_width(tile_width);
            }
            "tileheight" => {
                let tile_height = try!(reader::read_num(value));
                map.set_tile_height(tile_height);
            }
            "hexsidelength" => {
                let hex_side_length = try!(reader::read_num(value));
                map.set_hex_side_length(hex_side_length);
            }
            "staggeraxis" => {
                let stagger_axis = try!(Axis::from_str(value));
                map.set_stagger_axis(stagger_axis);
            }
            "staggerindex" => {
                let stagger_index = try!(Index::from_str(value));
                map.set_stagger_index(stagger_index);
            }
            "backgroundcolor" => {
                let color = try!(Color::from_str(value));
                map.set_background_color(color);
            }
            "nextobjectid" => {
                let next_object_id = try!(reader::read_num(value));
                map.set_next_object_id(next_object_id);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, map: &mut Map, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "tileset" => {
                let ts = try!(self.on_tileset(attributes));
                map.add_tileset(ts);
            }
            "layer" => {
                let layer = try!(self.on_layer(attributes));
                map.add_layer(layer);
            }
            "objectgroup" => {
                let object_group = try!(self.on_object_group(attributes));
                map.add_object_group(object_group);
            }
            "imagelayer" => {
                let image_layer = try!(self.on_image_layer(attributes));
                map.add_image_layer(image_layer);
            }
            _ => {}
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<Layer> for TmxReader<R> {
    fn read_attributes(&mut self, layer: &mut Layer, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                layer.set_name(value);
            }
            "x" => {
                let x = try!(reader::read_num(value));
                layer.set_x(x);
            }
            "y" => {
                let y = try!(reader::read_num(value));
                layer.set_y(y);
            }
            "width" => {
                let width = try!(reader::read_num(value));
                layer.set_width(width);
            }
            "height" => {
                let height = try!(reader::read_num(value));
                layer.set_height(height);
            }
            "opacity" => {
                let opacity = try!(reader::read_num(value));
                layer.set_opacity(opacity);
            }
            "visible" => {
                let visibility = try!(reader::read_num::<u32>(value));
                if visibility == 0 {
                    layer.set_visible(false);
                }
            }
            "offsetx" => {
                let offset_x = try!(reader::read_num(value));
                layer.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(reader::read_num(value));
                layer.set_offset_y(offset_y);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, layer: &mut Layer, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                layer.set_properties(properties);
            }
            "data" => {
                let data = try!(self.on_data(attributes));
                layer.set_data(data);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<ImageLayer> for TmxReader<R> {
    fn read_attributes(&mut self, image_layer: &mut ImageLayer, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                image_layer.set_name(value);
            }
            "offsetx" => {
                let offset_x = try!(reader::read_num(value));
                image_layer.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(reader::read_num(value));
                image_layer.set_offset_y(offset_y);
            }
            "x" => {
                let x = try!(reader::read_num(value));
                image_layer.set_x(x);
            }
            "y" => {
                let y = try!(reader::read_num(value));
                image_layer.set_y(y);
            }
            "width" => {
                let width = try!(reader::read_num(value));
                image_layer.set_width(width);
            }
            "height" => {
                let height = try!(reader::read_num(value));
                image_layer.set_height(height);
            }
            "opacity" => {
                let opacity = try!(reader::read_num(value));
                image_layer.set_opacity(opacity);
            }
            "visible" => {
                let visibility = try!(reader::read_num::<u32>(value));
                if visibility == 0 {
                    image_layer.set_visible(false);
                }
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, image_layer: &mut ImageLayer, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                image_layer.set_properties(properties);
            }
            "image" => {
                let image = try!(self.on_image(attributes));
                image_layer.set_image(image);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<ObjectGroup> for TmxReader<R> {
    fn read_attributes(&mut self, object_group: &mut ObjectGroup, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                object_group.set_name(value);
            }
            "color" => {
                let color = try!(Color::from_str(value));
                object_group.set_color(color);
            }
            "x" => {
                let x = try!(reader::read_num(value));
                object_group.set_x(x);
            }
            "y" => {
                let y = try!(reader::read_num(value));
                object_group.set_y(y);
            }
            "width" => {
                let width = try!(reader::read_num(value));
                object_group.set_width(width);
            }
            "height" => {
                let height = try!(reader::read_num(value));
                object_group.set_height(height);
            }
            "opacity" => {
                let opacity = try!(reader::read_num(value));
                object_group.set_opacity(opacity);
            }
            "visible" => {
                let visibility = try!(reader::read_num::<u32>(value));
                if visibility == 0 {
                    object_group.set_visible(false);
                }
            }
            "offsetx" => {
                let offset_x = try!(reader::read_num(value));
                object_group.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(reader::read_num(value));
                object_group.set_offset_y(offset_y);
            }
            "draworder" => {
                let draw_order = try!(DrawOrder::from_str(value));
                object_group.set_draw_order(draw_order);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, object_group: &mut ObjectGroup, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                object_group.set_properties(properties);
            }
            "object" => {
                let object = try!(self.on_object(attributes));
                object_group.add_object(object);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<Object> for TmxReader<R> {
    fn read_attributes(&mut self, object: &mut Object, name: &str, value: &str) -> ::Result<()> {
        match name {
            "id" => {
                let id = try!(reader::read_num(value));
                object.set_id(id);
            }
            "name" => {
                object.set_name(value);
            }
            "type" => {
                object.set_object_type(value);
            }
            "x" => {
                let x = try!(reader::read_num(value));
                object.set_x(x);
            }
            "y" => {
                let y = try!(reader::read_num(value));
                object.set_y(y);
            }
            "width" => {
                let width = try!(reader::read_num(value));
                object.set_width(width);
            }
            "height" => {
                let height = try!(reader::read_num(value));
                object.set_height(height);
            }
            "rotation" => {
                let rotation = try!(reader::read_num(value));
                object.set_rotation(rotation);
            }
            "gid" => {
                let gid = try!(reader::read_num(value));
                object.set_gid(gid);
            }
            "visible" => {
                let visibility = try!(reader::read_num::<u32>(value));
                if visibility == 0 {
                    object.set_visible(false);
                }
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, object: &mut Object, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                object.set_properties(properties);
            }
            "ellipse" => {
                object.set_shape(Shape::Ellipse);
            }
            "polygon" => {
                let polygon = try!(self.on_polygon(attributes));
                object.set_shape(polygon);
            }
            "polyline" => {
                let polyline = try!(self.on_polyline(attributes));
                object.set_shape(polyline);
            }
            _ => {}
        };
        Ok(())
    }
}

