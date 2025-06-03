#[doc = "Register `BGCOLR` reader"]
pub type R = crate::R<BgcolrSpec>;
#[doc = "Register `BGCOLR` writer"]
pub type W = crate::W<BgcolrSpec>;
#[doc = "Field `BLUE` reader - Blue Value These bits define the blue value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type BlueR = crate::FieldReader;
#[doc = "Field `BLUE` writer - Blue Value These bits define the blue value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type BlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` reader - Green Value These bits define the green value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type GreenR = crate::FieldReader;
#[doc = "Field `GREEN` writer - Green Value These bits define the green value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type GreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED` reader - Red Value These bits define the red value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type RedR = crate::FieldReader;
#[doc = "Field `RED` writer - Red Value These bits define the red value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type RedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Blue Value These bits define the blue value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn blue(&self) -> BlueR {
        BlueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green Value These bits define the green value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn green(&self) -> GreenR {
        GreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Red Value These bits define the red value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn red(&self) -> RedR {
        RedR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blue Value These bits define the blue value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn blue(&mut self) -> BlueW<BgcolrSpec> {
        BlueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green Value These bits define the green value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn green(&mut self) -> GreenW<BgcolrSpec> {
        GreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red Value These bits define the red value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn red(&mut self) -> RedW<BgcolrSpec> {
        RedW::new(self, 16)
    }
}
#[doc = "DMA2D background color register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcolr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgcolrSpec;
impl crate::RegisterSpec for BgcolrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgcolr::R`](R) reader structure"]
impl crate::Readable for BgcolrSpec {}
#[doc = "`write(|w| ..)` method takes [`bgcolr::W`](W) writer structure"]
impl crate::Writable for BgcolrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BGCOLR to value 0"]
impl crate::Resettable for BgcolrSpec {}
