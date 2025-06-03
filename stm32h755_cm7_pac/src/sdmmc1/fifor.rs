#[doc = "Register `FIFOR` reader"]
pub type R = crate::R<FiforSpec>;
#[doc = "Register `FIFOR` writer"]
pub type W = crate::W<FiforSpec>;
#[doc = "Field `FIFODATA` reader - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
pub type FifodataR = crate::FieldReader<u32>;
#[doc = "Field `FIFODATA` writer - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
pub type FifodataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
    #[inline(always)]
    pub fn fifodata(&self) -> FifodataR {
        FifodataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
    #[inline(always)]
    pub fn fifodata(&mut self) -> FifodataW<FiforSpec> {
        FifodataW::new(self, 0)
    }
}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiforSpec;
impl crate::RegisterSpec for FiforSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifor::R`](R) reader structure"]
impl crate::Readable for FiforSpec {}
#[doc = "`write(|w| ..)` method takes [`fifor::W`](W) writer structure"]
impl crate::Writable for FiforSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOR to value 0"]
impl crate::Resettable for FiforSpec {}
