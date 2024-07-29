use transcribe::validate_path;

#[test]
fn validate_upload_paths() {
    // Case 0: Valid path
    assert!(validate_path("some/root/path/week1/lesson1/video0.mp4").is_ok());

    // Case 1: Invalid video suffix
    let err = validate_path("some/root/path/week1/lesson1/videoX.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/week1/lesson1/videoX.mp4. Video id must be strictly numbered i.e **/video##.mp4"
    );
    let err = validate_path("some/root/path/week1/lesson1/video_0.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/week1/lesson1/video_0.mp4. Video id must be strictly numbered i.e **/video##.mp4"
    );

    // Case 2: Invalid lesson suffix
    let err = validate_path("some/root/path/week1/lesson_1/video0.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/week1/lesson_1/video0.mp4. Videos must be strictly within 'lesson##' directory i.e. **/lesson##/video##.mp4"
    );
    let err = validate_path("some/root/path/week1/lessonX/video0.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/week1/lessonX/video0.mp4. Videos must be strictly within 'lesson##' directory i.e. **/lesson##/video##.mp4"
    );

    // Case 3: Invalid week suffix
    let err = validate_path("some/root/path/week_1/lesson1/video0.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/week_1/lesson1/video0.mp4. Videos must be strictly within 'week##/lesson##' directory i.e. */week##/lesson##/video##.mp4"
    );
    let err = validate_path("some/root/path/weekX/lesson1/video0.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/weekX/lesson1/video0.mp4. Videos must be strictly within 'week##/lesson##' directory i.e. */week##/lesson##/video##.mp4"
    );

    //Case 4: Invalid directory structures
    let err = validate_path("some/root/path/week1/video0.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/week1/video0.mp4. Videos must be strictly within 'lesson##' directory i.e. **/lesson##/video##.mp4"
    );
    let err = validate_path("some/root/path/lesson1/video0.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/lesson1/video0.mp4. Videos must be strictly within 'week##/lesson##' directory i.e. */week##/lesson##/video##.mp4"
    );
    let err = validate_path("some/root/path/lesson1/week1/video0.mp4").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid path format some/root/path/lesson1/week1/video0.mp4. Videos must be strictly within 'lesson##' directory i.e. **/lesson##/video##.mp4"
    );
}
