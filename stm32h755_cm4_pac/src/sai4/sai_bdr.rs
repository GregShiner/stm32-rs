#[doc = "Register `SAI_BDR` reader"]
pub type R = crate::R<SaiBdrSpec>;
#[doc = "Register `SAI_BDR` writer"]
pub type W = crate::W<SaiBdrSpec>;
#[doc = "Field `DATA` reader - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<SaiBdrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`sai_bdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_bdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaiBdrSpec;
impl crate::RegisterSpec for SaiBdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_bdr::R`](R) reader structure"]
impl crate::Readable for SaiBdrSpec {}
#[doc = "`write(|w| ..)` method takes [`sai_bdr::W`](W) writer structure"]
impl crate::Writable for SaiBdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAI_BDR to value 0"]
impl crate::Resettable for SaiBdrSpec {}
