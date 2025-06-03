#[doc = "Register `FGOR` reader"]
pub type R = crate::R<FgorSpec>;
#[doc = "Register `FGOR` writer"]
pub type W = crate::W<FgorSpec>;
#[doc = "Field `LO` reader - Line offset Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
pub type LoR = crate::FieldReader<u16>;
#[doc = "Field `LO` writer - Line offset Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
pub type LoW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Line offset Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Line offset Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&mut self) -> LoW<FgorSpec> {
        LoW::new(self, 0)
    }
}
#[doc = "DMA2D foreground offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FgorSpec;
impl crate::RegisterSpec for FgorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fgor::R`](R) reader structure"]
impl crate::Readable for FgorSpec {}
#[doc = "`write(|w| ..)` method takes [`fgor::W`](W) writer structure"]
impl crate::Writable for FgorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FGOR to value 0"]
impl crate::Resettable for FgorSpec {}
