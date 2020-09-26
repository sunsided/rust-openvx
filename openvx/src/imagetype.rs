use libopenvx_sys::vx_df_image_e;

pub mod constants {
    use libopenvx_sys::vx_df_image_e;

    /// Converts a set of four chars into a `uint32_t` container of a VX_DF_IMAGE code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openvx::{vx_df_image, constants::images};
    ///
    /// let t = vx_df_image!('R', 'G', 'B', 'A');
    /// assert_eq!(t, images::VX_DF_IMAGE_RGBX);
    /// ```
    #[macro_export]
    macro_rules! vx_df_image {
        ($a:literal, $b:literal, $c:literal, $d:literal) => {{
            use libopenvx_sys::{vx_df_image_e, vx_uint32, vx_uint8};
            let a: vx_uint32 = $a as vx_uint8 as vx_uint32;
            let b: vx_uint32 = ($b as vx_uint8 as vx_uint32) << 8;
            let c: vx_uint32 = ($c as vx_uint8 as vx_uint32) << 16;
            let d: vx_uint32 = ($d as vx_uint8 as vx_uint32) << 24;
            (a | b | c | d) as vx_df_image_e
        }};
    }

    /// A virtual image of no defined type.
    pub const VX_DF_IMAGE_VIRT: vx_df_image_e = vx_df_image!('V', 'I', 'R', 'T');

    /// A single plane of 24-bit pixel as 3 interleaved 8-bit units of
    /// R then G then B data. This uses the BT709 full range by default.
    pub const VX_DF_IMAGE_RGB: vx_df_image_e = vx_df_image!('R', 'G', 'B', '2');

    /// A single plane of 32-bit pixel as 4 interleaved 8-bit units of
    /// R then G then B data, then a _don't care_ byte.
    /// This uses the BT709 full range by default.
    pub const VX_DF_IMAGE_RGBX: vx_df_image_e = vx_df_image!('R', 'G', 'B', 'A');

    /// A 2-plane YUV format of Luma (Y) and interleaved UV data at
    /// 4:2:0 sampling. This uses the BT709 full range by default.
    pub const VX_DF_IMAGE_NV12: vx_df_image_e = vx_df_image!('N', 'V', '1', '2');

    /// A 2-plane YUV format of Luma (Y) and interleaved VU data at
    /// 4:2:0 sampling. This uses the BT709 full range by default.
    pub const VX_DF_IMAGE_NV21: vx_df_image_e = vx_df_image!('N', 'V', '2', '1');

    /// A single plane of 32-bit macro pixel of U0, Y0, V0, Y1 bytes.
    /// This uses the BT709 full range by default.
    pub const VX_DF_IMAGE_UYVY: vx_df_image_e = vx_df_image!('U', 'Y', 'V', 'Y');

    /// A single plane of 32-bit macro pixel of Y0, U0, Y1, V0 bytes.
    /// This uses the BT709 full range by default.
    pub const VX_DF_IMAGE_YUYV: vx_df_image_e = vx_df_image!('Y', 'U', 'Y', 'V');

    /// A 3 plane of 8-bit 4:2:0 sampled Y, U, V planes.
    /// This uses the BT709 full range by default.
    pub const VX_DF_IMAGE_IYUV: vx_df_image_e = vx_df_image!('I', 'Y', 'U', 'V');

    /// A 3 plane of 8 bit 4:4:4 sampled Y, U, V planes.
    /// This uses the BT709 full range by default.
    pub const VX_DF_IMAGE_YUV4: vx_df_image_e = vx_df_image!('Y', 'U', 'V', '4');

    /// A single plane of unsigned 1-bit data packed eight pixels per byte.
    /// The least significant bit is the first pixel in each byte.
    /// TODO: See <tt>\\ref vx_imagepatch_addressing_t</tt> for more details.
    pub const VX_DF_IMAGE_U1: vx_df_image_e = vx_df_image!('U', '0', '0', '1');

    /// A single plane of unsigned 8-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    pub const VX_DF_IMAGE_U8: vx_df_image_e = vx_df_image!('U', '0', '0', '8');

    /// A single plane of signed 16-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    pub const VX_DF_IMAGE_U16: vx_df_image_e = vx_df_image!('U', '0', '1', '6');

    /// A single plane of signed 16-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    pub const VX_DF_IMAGE_S16: vx_df_image_e = vx_df_image!('S', '0', '1', '6');

    /// A single plane of unsigned 32-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    pub const VX_DF_IMAGE_U32: vx_df_image_e = vx_df_image!('U', '0', '3', '2');

    /// A single plane of signed 32-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    pub const VX_DF_IMAGE_S32: vx_df_image_e = vx_df_image!('S', '0', '3', '2');

    #[cfg(test)]
    mod tests {
        use super::*;
        use libopenvx_sys::*;

        #[test]
        fn type_virt() {
            assert_eq!(VX_DF_IMAGE_VIRT, vx_df_image_e_VX_DF_IMAGE_VIRT);
        }

        #[test]
        fn types_rgb() {
            assert_eq!(VX_DF_IMAGE_RGB, vx_df_image_e_VX_DF_IMAGE_RGB);
        }

        #[test]
        fn types_rgbx() {
            assert_eq!(VX_DF_IMAGE_RGBX, vx_df_image_e_VX_DF_IMAGE_RGBX);
        }

        #[test]
        fn types_nv12() {
            assert_eq!(VX_DF_IMAGE_NV12, vx_df_image_e_VX_DF_IMAGE_NV12);
        }

        #[test]
        fn types_nv21() {
            assert_eq!(VX_DF_IMAGE_NV21, vx_df_image_e_VX_DF_IMAGE_NV21);
        }

        #[test]
        fn types_uyvy() {
            assert_eq!(VX_DF_IMAGE_UYVY, vx_df_image_e_VX_DF_IMAGE_UYVY);
        }

        #[test]
        fn types_yuyv() {
            assert_eq!(VX_DF_IMAGE_YUYV, vx_df_image_e_VX_DF_IMAGE_YUYV);
        }

        #[test]
        fn types_iyuv() {
            assert_eq!(VX_DF_IMAGE_IYUV, vx_df_image_e_VX_DF_IMAGE_IYUV);
        }

        #[test]
        fn types_yuv4() {
            assert_eq!(VX_DF_IMAGE_YUV4, vx_df_image_e_VX_DF_IMAGE_YUV4);
        }

        #[test]
        fn types_u1() {
            assert_eq!(VX_DF_IMAGE_U1, vx_df_image_e_VX_DF_IMAGE_U1);
        }

        #[test]
        fn types_u8() {
            assert_eq!(VX_DF_IMAGE_U8, vx_df_image_e_VX_DF_IMAGE_U8);
        }

        #[test]
        fn types_u16() {
            assert_eq!(VX_DF_IMAGE_U16, vx_df_image_e_VX_DF_IMAGE_U16);
        }

        #[test]
        fn types_s16() {
            assert_eq!(VX_DF_IMAGE_S16, vx_df_image_e_VX_DF_IMAGE_S16);
        }

        #[test]
        fn types_u32() {
            assert_eq!(VX_DF_IMAGE_U32, vx_df_image_e_VX_DF_IMAGE_U32);
        }

        #[test]
        fn types_s32() {
            assert_eq!(VX_DF_IMAGE_S32, vx_df_image_e_VX_DF_IMAGE_S32);
        }
    }
}

/// Used to hold a `VX_DF_IMAGE` code to describe the pixel format and color space.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ImageType {
    /// A virtual image of no defined type.
    Virtual,
    /// A single plane of 24-bit pixel as 3 interleaved 8-bit units of
    /// R then G then B data. This uses the BT709 full range by default.
    RGB,
    /// A single plane of 32-bit pixel as 4 interleaved 8-bit units of
    /// R then G then B data, then a _don't care_ byte.
    /// This uses the BT709 full range by default.
    RGBX,
    /// A 2-plane YUV format of Luma (Y) and interleaved UV data at
    /// 4:2:0 sampling. This uses the BT709 full range by default.
    NV12,
    /// A 2-plane YUV format of Luma (Y) and interleaved VU data at
    /// 4:2:0 sampling. This uses the BT709 full range by default.
    NV21,
    /// A single plane of 32-bit macro pixel of U0, Y0, V0, Y1 bytes.
    /// This uses the BT709 full range by default.
    UYVY,
    /// A single plane of 32-bit macro pixel of Y0, U0, Y1, V0 bytes.
    /// This uses the BT709 full range by default.
    YUYV,
    /// A 3 plane of 8-bit 4:2:0 sampled Y, U, V planes.
    /// This uses the BT709 full range by default.
    IYUV,
    /// A 3 plane of 8 bit 4:4:4 sampled Y, U, V planes.
    /// This uses the BT709 full range by default.
    YUV4,
    /// A single plane of unsigned 1-bit data packed eight pixels per byte.
    /// The least significant bit is the first pixel in each byte.
    /// TODO: See <tt>\\ref vx_imagepatch_addressing_t</tt> for more details.
    U1,
    /// A single plane of unsigned 8-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    U8,
    /// A single plane of unsigned 16-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    U16,
    /// A single plane of signed 16-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    S16,
    /// A single plane of unsigned 32-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    U32,
    /// A single plane of signed 32-bit data.
    /// The range of data is not specified, as it may be extracted from a YUV or generated.
    S32,
    /// An unknown image type.
    Other { r#type: vx_df_image_e },
}

impl ImageType {
    pub fn new(directive: vx_df_image_e) -> Self {
        match directive {
            constants::VX_DF_IMAGE_VIRT => ImageType::Virtual,
            constants::VX_DF_IMAGE_RGB => ImageType::RGB,
            constants::VX_DF_IMAGE_RGBX => ImageType::RGBX,
            constants::VX_DF_IMAGE_NV12 => ImageType::NV12,
            constants::VX_DF_IMAGE_NV21 => ImageType::NV21,
            constants::VX_DF_IMAGE_UYVY => ImageType::UYVY,
            constants::VX_DF_IMAGE_YUYV => ImageType::YUYV,
            constants::VX_DF_IMAGE_IYUV => ImageType::IYUV,
            constants::VX_DF_IMAGE_YUV4 => ImageType::YUV4,
            constants::VX_DF_IMAGE_U1 => ImageType::U1,
            constants::VX_DF_IMAGE_U8 => ImageType::U8,
            constants::VX_DF_IMAGE_U16 => ImageType::U16,
            constants::VX_DF_IMAGE_S16 => ImageType::S16,
            constants::VX_DF_IMAGE_U32 => ImageType::U32,
            constants::VX_DF_IMAGE_S32 => ImageType::S32,
            other => ImageType::Other { r#type: other },
        }
    }

    /// Converts this instance into a [`vx_df_image_e`].
    ///
    /// [`vx_df_image_e`]: ../libopenvx_sys/type.vx_df_image_e.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use openvx::{ImageType, constants::images};
    ///
    /// let x: ImageType = ImageType::RGBX;
    /// assert_eq!(x.to_raw(), images::VX_DF_IMAGE_RGBX);
    /// ```
    pub const fn to_raw(&self) -> vx_df_image_e {
        match self {
            ImageType::Virtual => constants::VX_DF_IMAGE_VIRT,
            ImageType::RGB => constants::VX_DF_IMAGE_RGB,
            ImageType::RGBX => constants::VX_DF_IMAGE_RGBX,
            ImageType::NV12 => constants::VX_DF_IMAGE_NV12,
            ImageType::NV21 => constants::VX_DF_IMAGE_NV21,
            ImageType::UYVY => constants::VX_DF_IMAGE_UYVY,
            ImageType::YUYV => constants::VX_DF_IMAGE_YUYV,
            ImageType::IYUV => constants::VX_DF_IMAGE_IYUV,
            ImageType::YUV4 => constants::VX_DF_IMAGE_YUV4,
            ImageType::U1 => constants::VX_DF_IMAGE_U1,
            ImageType::U8 => constants::VX_DF_IMAGE_U8,
            ImageType::U16 => constants::VX_DF_IMAGE_U16,
            ImageType::S16 => constants::VX_DF_IMAGE_S16,
            ImageType::U32 => constants::VX_DF_IMAGE_U32,
            ImageType::S32 => constants::VX_DF_IMAGE_S32,
            ImageType::Other { r#type } => *r#type,
        }
    }
}

impl From<vx_df_image_e> for ImageType {
    fn from(r#type: vx_df_image_e) -> Self {
        ImageType::new(r#type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_virt() {
        assert_eq!(ImageType::Virtual.to_raw(), constants::VX_DF_IMAGE_VIRT);
        assert_eq!(
            ImageType::from(constants::VX_DF_IMAGE_VIRT),
            ImageType::Virtual
        );
    }

    #[test]
    fn types_rgb() {
        assert_eq!(ImageType::RGB.to_raw(), constants::VX_DF_IMAGE_RGB);
        assert_eq!(ImageType::from(constants::VX_DF_IMAGE_RGB), ImageType::RGB);
    }

    #[test]
    fn types_rgbx() {
        assert_eq!(ImageType::RGBX.to_raw(), constants::VX_DF_IMAGE_RGBX);
        assert_eq!(
            ImageType::from(constants::VX_DF_IMAGE_RGBX),
            ImageType::RGBX
        );
    }

    #[test]
    fn types_nv12() {
        assert_eq!(ImageType::NV12.to_raw(), constants::VX_DF_IMAGE_NV12);
        assert_eq!(
            ImageType::from(constants::VX_DF_IMAGE_NV12),
            ImageType::NV12
        );
    }

    #[test]
    fn types_nv21() {
        assert_eq!(ImageType::NV21.to_raw(), constants::VX_DF_IMAGE_NV21);
        assert_eq!(
            ImageType::from(constants::VX_DF_IMAGE_NV21),
            ImageType::NV21
        );
    }

    #[test]
    fn types_uyvy() {
        assert_eq!(ImageType::UYVY.to_raw(), constants::VX_DF_IMAGE_UYVY);
        assert_eq!(
            ImageType::from(constants::VX_DF_IMAGE_UYVY),
            ImageType::UYVY
        );
    }

    #[test]
    fn types_yuyv() {
        assert_eq!(ImageType::YUYV.to_raw(), constants::VX_DF_IMAGE_YUYV);
        assert_eq!(
            ImageType::from(constants::VX_DF_IMAGE_YUYV),
            ImageType::YUYV
        );
    }

    #[test]
    fn types_iyuv() {
        assert_eq!(ImageType::IYUV.to_raw(), constants::VX_DF_IMAGE_IYUV);
        assert_eq!(
            ImageType::from(constants::VX_DF_IMAGE_IYUV),
            ImageType::IYUV
        );
    }

    #[test]
    fn types_yuv4() {
        assert_eq!(ImageType::YUV4.to_raw(), constants::VX_DF_IMAGE_YUV4);
        assert_eq!(
            ImageType::from(constants::VX_DF_IMAGE_YUV4),
            ImageType::YUV4
        );
    }

    #[test]
    fn types_u1() {
        assert_eq!(ImageType::U1.to_raw(), constants::VX_DF_IMAGE_U1);
        assert_eq!(ImageType::from(constants::VX_DF_IMAGE_U1), ImageType::U1);
    }

    #[test]
    fn types_u8() {
        assert_eq!(ImageType::U8.to_raw(), constants::VX_DF_IMAGE_U8);
        assert_eq!(ImageType::from(constants::VX_DF_IMAGE_U8), ImageType::U8);
    }

    #[test]
    fn types_u16() {
        assert_eq!(ImageType::U16.to_raw(), constants::VX_DF_IMAGE_U16);
        assert_eq!(ImageType::from(constants::VX_DF_IMAGE_U16), ImageType::U16);
    }

    #[test]
    fn types_s16() {
        assert_eq!(ImageType::S16.to_raw(), constants::VX_DF_IMAGE_S16);
        assert_eq!(ImageType::from(constants::VX_DF_IMAGE_S16), ImageType::S16);
    }

    #[test]
    fn types_u32() {
        assert_eq!(ImageType::U32.to_raw(), constants::VX_DF_IMAGE_U32);
        assert_eq!(ImageType::from(constants::VX_DF_IMAGE_U32), ImageType::U32);
    }

    #[test]
    fn types_s32() {
        assert_eq!(ImageType::S32.to_raw(), constants::VX_DF_IMAGE_S32);
        assert_eq!(ImageType::from(constants::VX_DF_IMAGE_S32), ImageType::S32);
    }
}
