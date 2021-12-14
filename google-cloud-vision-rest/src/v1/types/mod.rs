pub mod annotate_image_request;
pub mod annotate_image_response;
pub mod batch_annotate_images_response;
pub mod feature;
pub mod image_context;

pub use annotate_image_request::AnnotateImageRequest;
pub use annotate_image_response::AnnotateImageResponse;
pub use batch_annotate_images_response::BatchAnnotateImagesResponse;
pub use feature::Feature;
pub use image_context::ImageContext;
