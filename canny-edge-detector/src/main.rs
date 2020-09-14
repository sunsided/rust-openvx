// canny-edge-detector sample from https://github.com/KhronosGroup/openvx-samples

use libopenvx_sys::*;
use openvx::*;

unsafe fn run() {
    let width: vx_uint32 = 480;
    let height: vx_uint32 = 360;

    let mut context = vxCreateContext();
    error_check_object(context as vx_reference);
    vxRegisterLogCallback(context, Some(log_callback), vx_bool_e_vx_false_e as i32);

    let mut graph = vxCreateGraph(context);
    error_check_object(graph as vx_reference);

    let mut input_rgb_image = vxCreateImage(context, width, height, vx_df_image_e_VX_DF_IMAGE_RGB);
    let mut output_filtered_image =
        vxCreateImage(context, width, height, vx_df_image_e_VX_DF_IMAGE_RGB);
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
    let mut lower: vx_int32 = 80;
    let mut upper: vx_int32 = 100;
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

    error_check_status(vxReleaseGraph(&mut graph));
    error_check_status(vxReleaseImage(&mut yuv_image));
    error_check_status(vxReleaseImage(&mut luma_image));
    error_check_status(vxReleaseImage(&mut input_rgb_image));
    error_check_status(vxReleaseImage(&mut output_filtered_image));
    error_check_status(vxReleaseContext(&mut context));
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
