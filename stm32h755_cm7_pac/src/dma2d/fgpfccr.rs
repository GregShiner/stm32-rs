#[doc = "Register `FGPFCCR` reader"]
pub type R = crate::R<FgpfccrSpec>;
#[doc = "Register `FGPFCCR` writer"]
pub type W = crate::W<FgpfccrSpec>;
#[doc = "Field `CM` reader - Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type CmR = crate::FieldReader;
#[doc = "Field `CM` writer - Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type CmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCM` reader - CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
pub type CcmR = crate::BitReader;
#[doc = "Field `CCM` writer - CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
pub type CcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer)."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer)."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\] + 1."]
pub type CsR = crate::FieldReader;
#[doc = "Field `CS` writer - CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\] + 1."]
pub type CsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AM` reader - Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless"]
pub type AmR = crate::FieldReader;
#[doc = "Field `AM` writer - Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless"]
pub type AmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSS` reader - Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless"]
pub type CssR = crate::FieldReader;
#[doc = "Field `CSS` writer - Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless"]
pub type CssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AI` reader - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AiR = crate::BitReader;
#[doc = "Field `AI` writer - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBS` reader - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RbsR = crate::BitReader;
#[doc = "Field `RBS` writer - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALPHA` reader - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\\[1:0\\] bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only."]
pub type AlphaR = crate::FieldReader;
#[doc = "Field `ALPHA` writer - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\\[1:0\\] bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only."]
pub type AlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ccm(&self) -> CcmR {
        CcmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer)."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\] + 1."]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless"]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ai(&self) -> AiR {
        AiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn rbs(&self) -> RbsR {
        RbsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\\[1:0\\] bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only."]
    #[inline(always)]
    pub fn alpha(&self) -> AlphaR {
        AlphaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<FgpfccrSpec> {
        CmW::new(self, 0)
    }
    #[doc = "Bit 4 - CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ccm(&mut self) -> CcmW<FgpfccrSpec> {
        CcmW::new(self, 4)
    }
    #[doc = "Bit 5 - Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer)."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<FgpfccrSpec> {
        StartW::new(self, 5)
    }
    #[doc = "Bits 8:15 - CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\] + 1."]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<FgpfccrSpec> {
        CsW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless"]
    #[inline(always)]
    pub fn am(&mut self) -> AmW<FgpfccrSpec> {
        AmW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless"]
    #[inline(always)]
    pub fn css(&mut self) -> CssW<FgpfccrSpec> {
        CssW::new(self, 18)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ai(&mut self) -> AiW<FgpfccrSpec> {
        AiW::new(self, 20)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn rbs(&mut self) -> RbsW<FgpfccrSpec> {
        RbsW::new(self, 21)
    }
    #[doc = "Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\\[1:0\\] bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only."]
    #[inline(always)]
    pub fn alpha(&mut self) -> AlphaW<FgpfccrSpec> {
        AlphaW::new(self, 24)
    }
}
#[doc = "DMA2D foreground PFC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgpfccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgpfccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FgpfccrSpec;
impl crate::RegisterSpec for FgpfccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fgpfccr::R`](R) reader structure"]
impl crate::Readable for FgpfccrSpec {}
#[doc = "`write(|w| ..)` method takes [`fgpfccr::W`](W) writer structure"]
impl crate::Writable for FgpfccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FGPFCCR to value 0"]
impl crate::Resettable for FgpfccrSpec {}
