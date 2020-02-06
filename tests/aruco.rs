#![cfg(feature = "contrib")]

use opencv::{aruco::DetectorParameters, Result};

#[test]
fn aruco_detector_parameters() -> Result<()> {
    let mut ptr = DetectorParameters::create()?;
    let mut plain = DetectorParameters::default()?;

    assert_eq!(plain.adaptive_thresh_win_size_min()?, ptr.get().adaptive_thresh_win_size_min()?);
    ptr.get_mut().set_adaptive_thresh_win_size_min(4)?;
    assert_eq!(ptr.get().adaptive_thresh_win_size_min()?, 4);

    assert_eq!(plain.adaptive_thresh_win_size_max()?, ptr.get().adaptive_thresh_win_size_max()?);
    ptr.get_mut().set_adaptive_thresh_win_size_max(24)?;
    assert_eq!(ptr.get().adaptive_thresh_win_size_max()?, 24);

    assert_eq!(plain.adaptive_thresh_win_size_step()?, ptr.get().adaptive_thresh_win_size_step()?);
    ptr.get_mut().set_adaptive_thresh_win_size_step(11)?;
    assert_eq!(ptr.get().adaptive_thresh_win_size_step()?, 11);

    assert_eq!(plain.adaptive_thresh_constant()?, ptr.get().adaptive_thresh_constant()?);
    ptr.get_mut().set_adaptive_thresh_constant(8.0)?;
    assert_eq!(ptr.get().adaptive_thresh_constant()?, 8.0);

    assert_eq!(plain.min_marker_perimeter_rate()?, ptr.get().min_marker_perimeter_rate()?);
    ptr.get_mut().set_min_marker_perimeter_rate(1.0)?;
    assert_eq!(ptr.get().min_marker_perimeter_rate()?, 1.0);

    assert_eq!(plain.max_marker_perimeter_rate()?, ptr.get().max_marker_perimeter_rate()?);
    ptr.get_mut().set_max_marker_perimeter_rate(5.0)?;
    assert_eq!(ptr.get().max_marker_perimeter_rate()?, 5.0);

    assert_eq!(plain.min_corner_distance_rate()?, ptr.get().min_corner_distance_rate()?);
    ptr.get_mut().set_min_corner_distance_rate(1.0)?;
    assert_eq!(ptr.get().min_corner_distance_rate()?, 1.0);

    assert_eq!(plain.min_distance_to_border()?, ptr.get().min_distance_to_border()?);
    ptr.get_mut().set_min_distance_to_border(4)?;
    assert_eq!(ptr.get().min_distance_to_border()?, 4);

    assert_eq!(plain.min_marker_distance_rate()?, ptr.get().min_marker_distance_rate()?);
    ptr.get_mut().set_min_marker_distance_rate(1.0)?;
    assert_eq!(ptr.get().min_marker_distance_rate()?, 1.0);

    assert_eq!(plain.corner_refinement_win_size()?, ptr.get().corner_refinement_win_size()?);
    ptr.get_mut().set_corner_refinement_win_size(6)?;
    assert_eq!(ptr.get().corner_refinement_win_size()?, 6);

    assert_eq!(plain.corner_refinement_max_iterations()?, ptr.get().corner_refinement_max_iterations()?);
    ptr.get_mut().set_corner_refinement_max_iterations(31)?;
    assert_eq!(ptr.get().corner_refinement_max_iterations()?, 31);

    assert_eq!(plain.corner_refinement_min_accuracy()?, ptr.get().corner_refinement_min_accuracy()?);
    ptr.get_mut().set_corner_refinement_min_accuracy(1.0)?;
    assert_eq!(ptr.get().corner_refinement_min_accuracy()?, 1.0);

    assert_eq!(plain.marker_border_bits()?, ptr.get().marker_border_bits()?);
    ptr.get_mut().set_marker_border_bits(2)?;
    assert_eq!(ptr.get().marker_border_bits()?, 2);

    assert_eq!(plain.perspective_remove_ignored_margin_per_cell()?, ptr.get().perspective_remove_ignored_margin_per_cell()?);
    ptr.get_mut().set_perspective_remove_ignored_margin_per_cell(1.0)?;
    assert_eq!(ptr.get().perspective_remove_ignored_margin_per_cell()?, 1.0);

    assert_eq!(plain.max_erroneous_bits_in_border_rate()?, ptr.get().max_erroneous_bits_in_border_rate()?);
    ptr.get_mut().set_max_erroneous_bits_in_border_rate(1.0)?;
    assert_eq!(ptr.get().max_erroneous_bits_in_border_rate()?, 1.0);

    assert_eq!(plain.min_otsu_std_dev()?, ptr.get().min_otsu_std_dev()?);
    ptr.get_mut().set_min_otsu_std_dev(6.0)?;
    assert_eq!(ptr.get().min_otsu_std_dev()?, 6.0);

    assert_eq!(plain.error_correction_rate()?, ptr.get().error_correction_rate()?);
    ptr.get_mut().set_error_correction_rate(1.0)?;
    assert_eq!(ptr.get().error_correction_rate()?, 1.0);

    assert_eq!(plain.detect_inverted_marker()?, ptr.get().detect_inverted_marker()?);
    ptr.get_mut().set_detect_inverted_marker(true)?;
    assert_eq!(ptr.get().detect_inverted_marker()?, true);

    assert_eq!(plain.perspective_remove_pixel_per_cell()?, ptr.get().perspective_remove_pixel_per_cell()?);
    ptr.get_mut().set_perspective_remove_pixel_per_cell(5)?;
    assert_eq!(ptr.get().perspective_remove_pixel_per_cell()?, 5);

    plain.set_adaptive_thresh_constant(123.)?;
    assert_eq!(123., plain.adaptive_thresh_constant()?);
    plain.set_adaptive_thresh_constant(87.)?;
    assert_eq!(87., plain.adaptive_thresh_constant()?);

    Ok(())
}
