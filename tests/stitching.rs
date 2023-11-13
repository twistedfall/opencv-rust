#![cfg(ocvrs_has_module_stitching)]

use opencv::core::Size;
use opencv::prelude::*;
use opencv::stitching::Detail_ImageFeatures;
use opencv::Result;

#[test]
fn default_ctor() -> Result<()> {
	let imgfeatures = Detail_ImageFeatures::default();
	assert_eq!(0, imgfeatures.img_idx());
	assert_eq!(Size::default(), imgfeatures.img_size());
	assert!(imgfeatures.keypoints().is_empty());
	assert_eq!(Size::default(), imgfeatures.descriptors().size()?);
	Ok(())
}
