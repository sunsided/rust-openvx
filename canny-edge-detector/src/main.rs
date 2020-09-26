// canny-edge-detector sample from https://github.com/KhronosGroup/openvx-samples

mod names;
mod print_attributes;

use crate::names::set_node_name;
use crate::print_attributes::{print_graph_attributes, print_node_attributes};
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

    let mut context = VxContext::create();
    context.check_status().expect("Context was invalid");
    context.enable_logging().expect("Unable to enable logging");
    context
        .enable_performance_counters()
        .expect("Unable to enable performance counters");

    let mut graph = context.create_graph();
    graph
        .check_status()
        .expect("Graph was invalid")
        .set_name("CANNY_GRAPH");

    let mut input_rgb_image = vxCreateImage(
        context.as_raw(),
        width,
        height,
        vx_df_image_e_VX_DF_IMAGE_RGB as vx_df_image,
    );
    let mut output_filtered_image = vxCreateImage(
        context.as_raw(),
        width,
        height,
        vx_df_image_e_VX_DF_IMAGE_U8 as vx_df_image,
    );
    error_check_object(input_rgb_image as vx_reference);
    error_check_object(output_filtered_image as vx_reference);

    let mut yuv_image = vxCreateVirtualImage(
        graph.as_raw(),
        width,
        height,
        vx_df_image_e_VX_DF_IMAGE_IYUV as vx_df_image,
    );
    let mut luma_image = vxCreateVirtualImage(
        graph.as_raw(),
        width,
        height,
        vx_df_image_e_VX_DF_IMAGE_U8 as vx_df_image,
    );
    error_check_object(yuv_image as vx_reference);
    error_check_object(luma_image as vx_reference);

    let hyst = vxCreateThreshold(
        context.as_raw(),
        vx_threshold_type_e_VX_THRESHOLD_TYPE_RANGE as vx_enum,
        vx_type_e_VX_TYPE_UINT8 as vx_enum,
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
        set_node_name(
            vxColorConvertNode(graph.as_raw(), input_rgb_image, yuv_image),
            "RGB_TO_YUV",
        ),
        set_node_name(
            vxChannelExtractNode(
                graph.as_raw(),
                yuv_image,
                vx_channel_e_VX_CHANNEL_Y as vx_enum,
                luma_image,
            ),
            "EXTRACT_LUMA",
        ),
        set_node_name(
            vxCannyEdgeDetectorNode(
                graph.as_raw(),
                luma_image,
                hyst,
                gradient_size,
                vx_norm_type_e_VX_NORM_L1 as vx_enum,
                output_filtered_image,
            ),
            "CANNY_EDGE",
        ),
    ];

    for node in nodes.iter_mut() {
        error_check_object(*node as vx_reference);
    }

    error_check_status(vxVerifyGraph(graph.as_raw()));

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
        start_x: 0 as vx_uint32,
        start_y: 0 as vx_uint32,
        end_x: width,
        end_y: height,
    };

    let cv_rgb_image_layout = vx_imagepatch_addressing_t {
        stride_x: 3 as vx_int32,
        stride_y: *resized.mat_step().first().unwrap() as vx_int32,
        // Default isn't implemented; not sure about these, but zeros seem to work.
        dim_x: 0 as vx_uint32,
        dim_y: 0 as vx_uint32,
        scale_x: 0 as vx_uint32,
        scale_y: 0 as vx_uint32,
        step_x: 0 as vx_uint32,
        step_y: 0 as vx_uint16,
        stride_x_bits: 0 as vx_uint16,
    };

    let cv_rgb_image_buffer: *mut vx_uint8 = resized.data_mut();
    error_check_status(vxCopyImagePatch(
        input_rgb_image,
        &cv_rgb_image_region,
        0,
        &cv_rgb_image_layout,
        cv_rgb_image_buffer as *mut std::ffi::c_void,
        vx_accessor_e_VX_WRITE_ONLY as vx_enum,
        vx_memory_type_e_VX_MEMORY_TYPE_HOST as vx_enum,
    ));

    error_check_status(vxProcessGraph(graph.as_raw()));

    let rect = vx_rectangle_t {
        start_x: 0 as vx_uint32,
        start_y: 0 as vx_uint32,
        end_x: width,
        end_y: height,
    };

    let mut map_id: vx_map_id = 0usize;
    let mut addr: vx_imagepatch_addressing_t = vx_imagepatch_addressing_t {
        dim_x: 0 as vx_uint32,
        dim_y: 0 as vx_uint32,
        stride_x: 0 as vx_int32,
        stride_y: 0 as vx_int32,
        scale_x: 0 as vx_uint32,
        scale_y: 0 as vx_uint32,
        step_x: 0 as vx_uint32,
        step_y: 0 as vx_uint16,
        stride_x_bits: 0 as vx_uint16,
    };
    let mut ptr: *mut std::ffi::c_void = std::ptr::null_mut();
    error_check_status(vxMapImagePatch(
        output_filtered_image,
        &rect,
        0 as vx_uint32,
        &mut map_id,
        &mut addr,
        &mut ptr,
        vx_accessor_e_VX_READ_ONLY as vx_enum,
        vx_memory_type_e_VX_MEMORY_TYPE_HOST as vx_enum,
        vx_map_flag_e_VX_NOGAP_X as vx_uint32,
    ));

    let mat = Mat::new_rows_cols_with_data(
        height as i32,
        width as i32,
        CV_8U,
        ptr,
        addr.stride_y as usize,
    )?;

    print_graph_attributes(&mut graph);
    for node in nodes.iter_mut() {
        print_node_attributes(*node);
        error_check_status(vxReleaseNode(node));
    }

    imshow("Canny Edge Detection", &mat)?;
    wait_key(0)?;
    destroy_all_windows()?;

    error_check_status(vxUnmapImagePatch(output_filtered_image, map_id));

    graph.release().expect("Releasing graph failed");
    error_check_status(vxReleaseImage(&mut yuv_image));
    error_check_status(vxReleaseImage(&mut luma_image));
    error_check_status(vxReleaseImage(&mut input_rgb_image));
    error_check_status(vxReleaseImage(&mut output_filtered_image));
    context.release().expect("Releasing context failed");

    Ok(())
}

fn error_check_object(r#ref: vx_reference) {
    let status = unsafe { vxGetStatus(r#ref) };
    let status = VxStatus::from(status);
    if status != VxStatus::Success {
        panic!("ERROR: failed with status {}", status);
    }
}

fn error_check_status(status: vx_status) {
    let status = VxStatus::from(status);
    if status != VxStatus::Success {
        panic!("ERROR: failed with status {}", status);
    }
}

fn main() {
    unsafe { run().unwrap() };
}
