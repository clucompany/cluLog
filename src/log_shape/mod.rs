
mod d_cluster;
mod d_color_cluster;


pub use self::d_cluster::*;
pub use self::d_color_cluster::*;


pub type DefLogShape = d_color_cluster::cluColorShape;

pub type DefLogNoColorShape = d_cluster::cluShape;
pub type DefLogColorShape = d_color_cluster::cluColorShape;

