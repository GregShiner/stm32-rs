#[doc = "Register `IDMABASE1R` reader"]
pub type R = crate::R<Idmabase1rSpec>;
#[doc = "Register `IDMABASE1R` writer"]
pub type W = crate::W<Idmabase1rSpec>;
#[doc = "Field `IDMABASE1` reader - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
pub type Idmabase1R = crate::FieldReader<u32>;
#[doc = "Field `IDMABASE1` writer - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
pub type Idmabase1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
    #[inline(always)]
    pub fn idmabase1(&self) -> Idmabase1R {
        Idmabase1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
    #[inline(always)]
    pub fn idmabase1(&mut self) -> Idmabase1W<Idmabase1rSpec> {
        Idmabase1W::new(self, 0)
    }
}
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.\n\nYou can [`read`](crate::Reg::read) this register and get [`idmabase1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabase1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idmabase1rSpec;
impl crate::RegisterSpec for Idmabase1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmabase1r::R`](R) reader structure"]
impl crate::Readable for Idmabase1rSpec {}
#[doc = "`write(|w| ..)` method takes [`idmabase1r::W`](W) writer structure"]
impl crate::Writable for Idmabase1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDMABASE1R to value 0"]
impl crate::Resettable for Idmabase1rSpec {}
