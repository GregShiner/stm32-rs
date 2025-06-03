#[doc = "Register `OOR` reader"]
pub type R = crate::R<OorSpec>;
#[doc = "Register `OOR` writer"]
pub type W = crate::W<OorSpec>;
#[doc = "Field `LO` reader - Line Offset Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LoR = crate::FieldReader<u16>;
#[doc = "Field `LO` writer - Line Offset Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LoW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Line Offset Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Line Offset Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn lo(&mut self) -> LoW<OorSpec> {
        LoW::new(self, 0)
    }
}
#[doc = "DMA2D output offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`oor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OorSpec;
impl crate::RegisterSpec for OorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oor::R`](R) reader structure"]
impl crate::Readable for OorSpec {}
#[doc = "`write(|w| ..)` method takes [`oor::W`](W) writer structure"]
impl crate::Writable for OorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OOR to value 0"]
impl crate::Resettable for OorSpec {}
