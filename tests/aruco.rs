#![cfg(feature = "contrib")]

use opencv::{aruco::DetectorParameters, Result};

#[test]
fn aruco_detector_parameters() -> Result<()> {
    let mut ptr = DetectorParameters::create()?;
    let mut params = ptr.deref_mut();

    assert_eq!(params.adaptive_thresh_win_size_min()?, 3);
    params.set_adaptive_thresh_win_size_min(4)?;
    assert_eq!(params.adaptive_thresh_win_size_min()?, 4);

    assert_eq!(params.adaptive_thresh_win_size_max()?, 23);
    params.set_adaptive_thresh_win_size_max(24)?;
    assert_eq!(params.adaptive_thresh_win_size_max()?, 24);

    assert_eq!(params.adaptive_thresh_win_size_step()?, 10);
    params.set_adaptive_thresh_win_size_step(11)?;
    assert_eq!(params.adaptive_thresh_win_size_step()?, 11);

    assert_eq!(params.adaptive_thresh_constant()?, 7.0);
    params.set_adaptive_thresh_constant(8.0)?;
    assert_eq!(params.adaptive_thresh_constant()?, 8.0);

    assert_eq!(params.min_marker_perimeter_rate()?, 0.03);
    params.set_min_marker_perimeter_rate(1.0)?;
    assert_eq!(params.min_marker_perimeter_rate()?, 1.0);

    assert_eq!(params.max_marker_perimeter_rate()?, 4.0);
    params.set_max_marker_perimeter_rate(5.0)?;
    assert_eq!(params.max_marker_perimeter_rate()?, 5.0);

    assert_eq!(params.min_corner_distance_rate()?, 0.05);
    params.set_min_corner_distance_rate(1.0)?;
    assert_eq!(params.min_corner_distance_rate()?, 1.0);

    assert_eq!(params.min_distance_to_border()?, 3);
    params.set_min_distance_to_border(4)?;
    assert_eq!(params.min_distance_to_border()?, 4);

    assert_eq!(params.min_marker_distance_rate()?, 0.05);
    params.set_min_marker_distance_rate(1.0)?;
    assert_eq!(params.min_marker_distance_rate()?, 1.0);

    assert_eq!(params.corner_refinement_win_size()?, 5);
    params.set_corner_refinement_win_size(6)?;
    assert_eq!(params.corner_refinement_win_size()?, 6);

    assert_eq!(params.corner_refinement_max_iterations()?, 30);
    params.set_corner_refinement_max_iterations(31)?;
    assert_eq!(params.corner_refinement_max_iterations()?, 31);

    assert_eq!(params.corner_refinement_min_accuracy()?, 0.1);
    params.set_corner_refinement_min_accuracy(1.0)?;
    assert_eq!(params.corner_refinement_min_accuracy()?, 1.0);

    assert_eq!(params.marker_border_bits()?, 1);
    params.set_marker_border_bits(2)?;
    assert_eq!(params.marker_border_bits()?, 2);

    assert_eq!(params.perspective_remove_ignored_margin_per_cell()?, 0.13);
    params.set_perspective_remove_ignored_margin_per_cell(1.0)?;
    assert_eq!(params.perspective_remove_ignored_margin_per_cell()?, 1.0);

    assert_eq!(params.max_erroneous_bits_in_border_rate()?, 0.35);
    params.set_max_erroneous_bits_in_border_rate(1.0)?;
    assert_eq!(params.max_erroneous_bits_in_border_rate()?, 1.0);

    assert_eq!(params.min_otsu_std_dev()?, 5.0);
    params.set_min_otsu_std_dev(6.0)?;
    assert_eq!(params.min_otsu_std_dev()?, 6.0);

    assert_eq!(params.error_correction_rate()?, 0.6);
    params.set_error_correction_rate(1.0)?;
    assert_eq!(params.error_correction_rate()?, 1.0);

    assert_eq!(params.detect_inverted_marker()?, false);
    params.set_detect_inverted_marker(true)?;
    assert_eq!(params.detect_inverted_marker()?, true);

    // documented default value is 8, but actually it's 4
    // see issue: https://github.com/opencv/opencv_contrib/issues/2420
    assert_eq!(params.perspective_remove_pixel_per_cell()?, 4);
    params.set_perspective_remove_pixel_per_cell(5)?;
    assert_eq!(params.perspective_remove_pixel_per_cell()?, 5);

    Ok(())
}
