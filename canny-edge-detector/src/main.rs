// canny-edge-detector sample from https://github.com/KhronosGroup/openvx-samples

use libopenvx_sys::*;
use opencv::core::CV_8U;
use opencv::{
    core::Size,
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};
use openvx::*;

type Result<T> = opencv::Result<T>;

unsafe fn run() -> Result<()> {
    let width: vx_uint32 = 512;
    let height: vx_uint32 = 512;

    let mut context = vxCreateContext();
    error_check_object(context as vx_reference);
    vxRegisterLogCallback(context, Some(log_callback), vx_bool_e_vx_false_e as i32);

    let mut graph = vxCreateGraph(context);
    error_check_object(graph as vx_reference);

    vxSetReferenceName(
        graph as vx_reference,
        std::ffi::CString::new("CANNY_GRAPH")
            .expect("CString::new failed")
            .as_ptr(),
    );

    let mut input_rgb_image = vxCreateImage(context, width, height, vx_df_image_e_VX_DF_IMAGE_RGB);
    let mut output_filtered_image =
        vxCreateImage(context, width, height, vx_df_image_e_VX_DF_IMAGE_U8);
    error_check_object(input_rgb_image as vx_reference);
    error_check_object(output_filtered_image as vx_reference);

    let mut yuv_image = vxCreateVirtualImage(graph, width, height, vx_df_image_e_VX_DF_IMAGE_IYUV);
    let mut luma_image = vxCreateVirtualImage(graph, width, height, vx_df_image_e_VX_DF_IMAGE_U8);
    error_check_object(yuv_image as vx_reference);
    error_check_object(luma_image as vx_reference);

    let hyst = vxCreateThreshold(
        context,
        vx_threshold_type_e_VX_THRESHOLD_TYPE_RANGE as i32,
        vx_type_e_VX_TYPE_UINT8 as i32,
    );
    let lower: vx_int32 = 130;
    let upper: vx_int32 = 150;
    vxSetThresholdAttribute(
        hyst,
        VX_THRESHOLD_ATTRIBUTE_THRESHOLD_LOWER,
        &lower as *const _ as *const std::ffi::c_void,
        std::mem::size_of_val(&lower) as vx_size,
    );
    vxSetThresholdAttribute(
        hyst,
        VX_THRESHOLD_ATTRIBUTE_THRESHOLD_UPPER,
        &upper as *const _ as *const std::ffi::c_void,
        std::mem::size_of_val(&upper) as vx_size,
    );
    error_check_object(hyst as vx_reference);

    let gradient_size: vx_int32 = 3;
    let mut nodes = vec![
        vxColorConvertNode(graph, input_rgb_image, yuv_image),
        vxChannelExtractNode(
            graph,
            yuv_image,
            vx_channel_e_VX_CHANNEL_Y as i32,
            luma_image,
        ),
        vxCannyEdgeDetectorNode(
            graph,
            luma_image,
            hyst,
            gradient_size,
            vx_norm_type_e_VX_NORM_L1 as i32,
            output_filtered_image,
        ),
    ];

    for node in nodes.iter_mut() {
        error_check_object(*node as vx_reference);
        error_check_status(vxReleaseNode(node));
    }

    error_check_status(vxVerifyGraph(graph));

    let image = imread(".images/selfie.jpg", IMREAD_COLOR)?;
    let mut resized = Mat::default()?;
    resize(
        &image,
        &mut resized,
        Size {
            width: width as i32,
            height: height as i32,
        },
        0f64,
        0f64,
        INTER_LINEAR,
    )?;

    imshow("Input Image", &resized)?;

    let cv_rgb_image_region = vx_rectangle_t {
        start_x: 0,
        start_y: 0,
        end_x: width,
        end_y: height,
    };

    let cv_rgb_image_layout = vx_imagepatch_addressing_t {
        stride_x: 3,
        stride_y: *resized.mat_step().first().unwrap() as i32,
        // Default isn't implemented; not sure about these, but zeros seem to work.
        dim_x: 0,
        dim_y: 0,
        scale_x: 0,
        scale_y: 0,
        step_x: 0,
        step_y: 0,
        stride_x_bits: 0,
    };

    let cv_rgb_image_buffer: *mut vx_uint8 = resized.data_mut();
    error_check_status(vxCopyImagePatch(
        input_rgb_image,
        &cv_rgb_image_region,
        0,
        &cv_rgb_image_layout,
        cv_rgb_image_buffer as *mut std::ffi::c_void,
        vx_accessor_e_VX_WRITE_ONLY as i32,
        vx_memory_type_e_VX_MEMORY_TYPE_HOST as i32,
    ));

    error_check_status(vxProcessGraph(graph));

    let rect = vx_rectangle_t {
        start_x: 0,
        start_y: 0,
        end_x: width,
        end_y: height,
    };

    let mut map_id: vx_map_id = 0usize;
    let mut addr: vx_imagepatch_addressing_t = vx_imagepatch_addressing_t {
        dim_x: 0,
        dim_y: 0,
        stride_x: 0,
        stride_y: 0,
        scale_x: 0,
        scale_y: 0,
        step_x: 0,
        step_y: 0,
        stride_x_bits: 0,
    };
    let mut ptr: *mut std::ffi::c_void = std::ptr::null_mut();
    error_check_status(vxMapImagePatch(
        output_filtered_image,
        &rect,
        0,
        &mut map_id,
        &mut addr,
        &mut ptr,
        vx_accessor_e_VX_READ_ONLY as i32,
        vx_memory_type_e_VX_MEMORY_TYPE_HOST as i32,
        vx_map_flag_e_VX_NOGAP_X,
    ));

    let mat = Mat::new_rows_cols_with_data(
        height as i32,
        width as i32,
        CV_8U,
        ptr,
        addr.stride_y as usize,
    )?;

    imshow("Canny Edge Detection", &mat)?;
    wait_key(0)?;
    destroy_all_windows()?;

    error_check_status(vxUnmapImagePatch(output_filtered_image, map_id));

    error_check_status(vxReleaseGraph(&mut graph));
    error_check_status(vxReleaseImage(&mut yuv_image));
    error_check_status(vxReleaseImage(&mut luma_image));
    error_check_status(vxReleaseImage(&mut input_rgb_image));
    error_check_status(vxReleaseImage(&mut output_filtered_image));
    error_check_status(vxReleaseContext(&mut context));

    Ok(())
}

#[allow(unused_variables)]
extern "C" fn log_callback(
    context: vx_context,
    r#ref: vx_reference,
    status: vx_status,
    string: *const vx_char,
) {
    debug_assert_ne!(status, vx_status_e_VX_SUCCESS);
    let status = VxStatus::from(status);

    if string.is_null() {
        return;
    }

    let string = unsafe { std::ffi::CStr::from_ptr(string) }
        .to_string_lossy()
        .into_owned();

    if string.len() == 0 {
        return;
    }

    eprint!("ERROR: {:#?} - {}", status, string);

    let require_linebreak = string.chars().last().unwrap() != '\n';
    if require_linebreak {
        eprintln!();
    }
}

fn error_check_object(r#ref: vx_reference) {
    let status = unsafe { vxGetStatus(r#ref) };
    let status = VxStatus::from(status);
    if status != VxStatus::Success {
        panic!("ERROR: failed with status {:#?}", status);
    }
}

fn error_check_status(status: vx_status) {
    let status = VxStatus::from(status);
    if status != VxStatus::Success {
        panic!("ERROR: failed with status {:#?}", status);
    }
}

fn main() {
    unsafe { run() };
}
