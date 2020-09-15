use libopenvx_sys::*;
use openvx::*;

pub unsafe fn print_graph_attributes(graph: vx_graph) {
    // http://software-dl.ti.com/jacinto7/esd/processor-sdk-rtos-jacinto7/latest/exports/docs/tiovx/docs/user_guide/vx__tutorial__image__color__convert_8c_source.html

    let mut num_nodes: vx_uint32 = 0;
    let mut num_params: vx_uint32 = 0;
    let mut ref_count: vx_uint32 = 0;
    let mut state: vx_enum = 0 as vx_enum;
    let mut perf = vx_perf_t {
        min: 0,
        max: 0,
        beg: 0,
        end: 0,
        num: 0,
        sum: 0,
        avg: 0,
        tmp: 0,
    };
    let mut ref_name: *mut vx_char = std::ptr::null_mut();

    vxQueryGraph(
        graph,
        vx_graph_attribute_e_VX_GRAPH_NUMNODES as vx_enum,
        &mut num_nodes as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&num_nodes) as vx_size,
    );

    vxQueryGraph(
        graph,
        vx_graph_attribute_e_VX_GRAPH_NUMPARAMETERS as vx_enum,
        &mut num_params as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&num_params) as vx_size,
    );

    vxQueryGraph(
        graph,
        vx_graph_attribute_e_VX_GRAPH_STATE as vx_enum,
        &mut state as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&state) as vx_size,
    );

    vxQueryGraph(
        graph,
        vx_graph_attribute_e_VX_GRAPH_PERFORMANCE as vx_enum,
        &mut perf as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&perf) as vx_size,
    );

    vxQueryReference(
        graph as vx_reference,
        vx_reference_attribute_e_VX_REFERENCE_NAME as vx_enum,
        &mut ref_name as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&ref_name) as vx_size,
    );

    vxQueryReference(
        graph as vx_reference,
        vx_reference_attribute_e_VX_REFERENCE_COUNT as vx_enum,
        &mut ref_count as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&ref_count) as vx_size,
    );

    let state = state as u32;
    #[allow(non_upper_case_globals)]
    let state_name = match state {
        vx_graph_state_e_VX_GRAPH_STATE_UNVERIFIED => "VX_GRAPH_STATE_UNVERIFIED",
        vx_graph_state_e_VX_GRAPH_STATE_VERIFIED => "VX_GRAPH_STATE_VERIFIED",
        vx_graph_state_e_VX_GRAPH_STATE_RUNNING => "VX_GRAPH_STATE_RUNNING",
        vx_graph_state_e_VX_GRAPH_STATE_ABANDONED => "VX_GRAPH_STATE_ABANDONED",
        vx_graph_state_e_VX_GRAPH_STATE_COMPLETED => "VX_GRAPH_STATE_COMPLETED",
        _ => "VX_GRAPH_STATE_UNKNOWN",
    };

    let ref_name = ref_name_from_cstr(ref_name);

    println!(
        " VX_TYPE_GRAPH: {}, {} nodes, {}, avg perf {}, {} parameters, {} refs",
        ref_name,
        num_nodes,
        state_name,
        perf.avg as f64 / 1000000000.0f64,
        num_params,
        ref_count
    );
}

pub unsafe fn print_node_attributes(node: vx_node) {
    // http://software-dl.ti.com/jacinto7/esd/processor-sdk-rtos-jacinto7/latest/exports/docs/tiovx/docs/user_guide/vx__tutorial__image__color__convert_8c_source.html

    let mut num_params: vx_uint32 = 0;
    let mut ref_count: vx_uint32 = 0;
    let mut status: vx_status = vx_status_e_VX_FAILURE;
    let mut perf = vx_perf_t {
        min: 0,
        max: 0,
        beg: 0,
        end: 0,
        num: 0,
        sum: 0,
        avg: 0,
        tmp: 0,
    };
    let mut ref_name: *mut vx_char = std::ptr::null_mut();

    vxQueryNode(
        node,
        vx_node_attribute_e_VX_NODE_STATUS as vx_enum,
        &mut status as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&status) as vx_size,
    );

    vxQueryNode(
        node,
        vx_node_attribute_e_VX_NODE_PARAMETERS as vx_enum,
        &mut num_params as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&num_params) as vx_size,
    );

    vxQueryNode(
        node,
        vx_node_attribute_e_VX_NODE_PERFORMANCE as vx_enum,
        &mut perf as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&perf) as vx_size,
    );

    vxQueryReference(
        node as vx_reference,
        vx_reference_attribute_e_VX_REFERENCE_NAME as vx_enum,
        &mut ref_name as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&ref_name) as vx_size,
    );

    vxQueryReference(
        node as vx_reference,
        vx_reference_attribute_e_VX_REFERENCE_COUNT as vx_enum,
        &mut ref_count as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of_val(&ref_count) as vx_size,
    );

    let status = VxStatus::from(status);

    let ref_name = ref_name_from_cstr(ref_name);

    println!(
        " VX_TYPE_NODE: {}, {} params, avg perf {}, {:#?}, {} refs",
        ref_name,
        num_params,
        perf.avg as f64 / 1000000000.0f64,
        status,
        ref_count
    );
}

unsafe fn ref_name_from_cstr(name: *const vx_char) -> String {
    if name.is_null() {
        String::from("INVALID_REF_NAME")
    } else {
        std::ffi::CStr::from_ptr(name)
            .to_string_lossy()
            .into_owned()
    }
}
