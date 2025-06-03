#[doc = "Register `IDMACTRLR` reader"]
pub type R = crate::R<IdmactrlrSpec>;
#[doc = "Register `IDMACTRLR` writer"]
pub type W = crate::W<IdmactrlrSpec>;
#[doc = "Field `IDMAEN` reader - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IdmaenR = crate::BitReader;
#[doc = "Field `IDMAEN` writer - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABMODE` reader - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IdmabmodeR = crate::BitReader;
#[doc = "Field `IDMABMODE` writer - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IdmabmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABACT` reader - Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware."]
pub type IdmabactR = crate::BitReader;
#[doc = "Field `IDMABACT` writer - Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware."]
pub type IdmabactW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmaen(&self) -> IdmaenR {
        IdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmabmode(&self) -> IdmabmodeR {
        IdmabmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware."]
    #[inline(always)]
    pub fn idmabact(&self) -> IdmabactR {
        IdmabactR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmaen(&mut self) -> IdmaenW<IdmactrlrSpec> {
        IdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmabmode(&mut self) -> IdmabmodeW<IdmactrlrSpec> {
        IdmabmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware."]
    #[inline(always)]
    pub fn idmabact(&mut self) -> IdmabactW<IdmactrlrSpec> {
        IdmabactW::new(self, 2)
    }
}
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`idmactrlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmactrlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdmactrlrSpec;
impl crate::RegisterSpec for IdmactrlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmactrlr::R`](R) reader structure"]
impl crate::Readable for IdmactrlrSpec {}
#[doc = "`write(|w| ..)` method takes [`idmactrlr::W`](W) writer structure"]
impl crate::Writable for IdmactrlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDMACTRLR to value 0"]
impl crate::Resettable for IdmactrlrSpec {}
