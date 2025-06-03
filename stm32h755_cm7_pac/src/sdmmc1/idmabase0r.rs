#[doc = "Register `IDMABASE0R` reader"]
pub type R = crate::R<Idmabase0rSpec>;
#[doc = "Register `IDMABASE0R` writer"]
pub type W = crate::W<Idmabase0rSpec>;
#[doc = "Field `IDMABASE0` reader - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
pub type Idmabase0R = crate::FieldReader<u32>;
#[doc = "Field `IDMABASE0` writer - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
pub type Idmabase0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
    #[inline(always)]
    pub fn idmabase0(&self) -> Idmabase0R {
        Idmabase0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
    #[inline(always)]
    pub fn idmabase0(&mut self) -> Idmabase0W<Idmabase0rSpec> {
        Idmabase0W::new(self, 0)
    }
}
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`idmabase0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabase0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idmabase0rSpec;
impl crate::RegisterSpec for Idmabase0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmabase0r::R`](R) reader structure"]
impl crate::Readable for Idmabase0rSpec {}
#[doc = "`write(|w| ..)` method takes [`idmabase0r::W`](W) writer structure"]
impl crate::Writable for Idmabase0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDMABASE0R to value 0"]
impl crate::Resettable for Idmabase0rSpec {}
