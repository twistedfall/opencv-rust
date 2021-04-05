#![cfg(ocvrs_has_module_aruco)]

use opencv::{aruco::DetectorParameters, prelude::*, Result};

#[test]
fn aruco_detector_parameters() -> Result<()> {
    let mut ptr = DetectorParameters::create()?;
    let mut plain = DetectorParameters::default()?;

    assert_eq!(plain.adaptive_thresh_win_size_min(), ptr.adaptive_thresh_win_size_min());
    ptr.set_adaptive_thresh_win_size_min(4);
    assert_eq!(ptr.adaptive_thresh_win_size_min(), 4);

    assert_eq!(plain.adaptive_thresh_win_size_max(), ptr.adaptive_thresh_win_size_max());
    ptr.set_adaptive_thresh_win_size_max(24);
    assert_eq!(ptr.adaptive_thresh_win_size_max(), 24);

    assert_eq!(plain.adaptive_thresh_win_size_step(), ptr.adaptive_thresh_win_size_step());
    ptr.set_adaptive_thresh_win_size_step(11);
    assert_eq!(ptr.adaptive_thresh_win_size_step(), 11);

    assert_eq!(plain.adaptive_thresh_constant(), ptr.adaptive_thresh_constant());
    ptr.set_adaptive_thresh_constant(8.0);
    assert_eq!(ptr.adaptive_thresh_constant(), 8.0);

    assert_eq!(plain.min_marker_perimeter_rate(), ptr.min_marker_perimeter_rate());
    ptr.set_min_marker_perimeter_rate(1.0);
    assert_eq!(ptr.min_marker_perimeter_rate(), 1.0);

    assert_eq!(plain.max_marker_perimeter_rate(), ptr.max_marker_perimeter_rate());
    ptr.set_max_marker_perimeter_rate(5.0);
    assert_eq!(ptr.max_marker_perimeter_rate(), 5.0);

    assert_eq!(plain.min_corner_distance_rate(), ptr.min_corner_distance_rate());
    ptr.set_min_corner_distance_rate(1.0);
    assert_eq!(ptr.min_corner_distance_rate(), 1.0);

    assert_eq!(plain.min_distance_to_border(), ptr.min_distance_to_border());
    ptr.set_min_distance_to_border(4);
    assert_eq!(ptr.min_distance_to_border(), 4);

    assert_eq!(plain.min_marker_distance_rate(), ptr.min_marker_distance_rate());
    ptr.set_min_marker_distance_rate(1.0);
    assert_eq!(ptr.min_marker_distance_rate(), 1.0);

    assert_eq!(plain.corner_refinement_win_size(), ptr.corner_refinement_win_size());
    ptr.set_corner_refinement_win_size(6);
    assert_eq!(ptr.corner_refinement_win_size(), 6);

    assert_eq!(plain.corner_refinement_max_iterations(), ptr.corner_refinement_max_iterations());
    ptr.set_corner_refinement_max_iterations(31);
    assert_eq!(ptr.corner_refinement_max_iterations(), 31);

    assert_eq!(plain.corner_refinement_min_accuracy(), ptr.corner_refinement_min_accuracy());
    ptr.set_corner_refinement_min_accuracy(1.0);
    assert_eq!(ptr.corner_refinement_min_accuracy(), 1.0);

    assert_eq!(plain.marker_border_bits(), ptr.marker_border_bits());
    ptr.set_marker_border_bits(2);
    assert_eq!(ptr.marker_border_bits(), 2);

    assert_eq!(plain.perspective_remove_ignored_margin_per_cell(), ptr.perspective_remove_ignored_margin_per_cell());
    ptr.set_perspective_remove_ignored_margin_per_cell(1.0);
    assert_eq!(ptr.perspective_remove_ignored_margin_per_cell(), 1.0);

    assert_eq!(plain.max_erroneous_bits_in_border_rate(), ptr.max_erroneous_bits_in_border_rate());
    ptr.set_max_erroneous_bits_in_border_rate(1.0);
    assert_eq!(ptr.max_erroneous_bits_in_border_rate(), 1.0);

    assert_eq!(plain.min_otsu_std_dev(), ptr.min_otsu_std_dev());
    ptr.set_min_otsu_std_dev(6.0);
    assert_eq!(ptr.min_otsu_std_dev(), 6.0);

    assert_eq!(plain.error_correction_rate(), ptr.error_correction_rate());
    ptr.set_error_correction_rate(1.0);
    assert_eq!(ptr.error_correction_rate(), 1.0);

    assert_eq!(plain.perspective_remove_pixel_per_cell(), ptr.perspective_remove_pixel_per_cell());
    ptr.set_perspective_remove_pixel_per_cell(5);
    assert_eq!(ptr.perspective_remove_pixel_per_cell(), 5);

    plain.set_adaptive_thresh_constant(123.);
    assert_eq!(123., plain.adaptive_thresh_constant());
    plain.set_adaptive_thresh_constant(87.);
    assert_eq!(87., plain.adaptive_thresh_constant());

    Ok(())
}
