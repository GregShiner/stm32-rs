#[doc = "Register `IDMABSIZER` reader"]
pub type R = crate::R<IdmabsizerSpec>;
#[doc = "Register `IDMABSIZER` writer"]
pub type W = crate::W<IdmabsizerSpec>;
#[doc = "Field `IDMABNDT` reader - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IdmabndtR = crate::FieldReader;
#[doc = "Field `IDMABNDT` writer - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IdmabndtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmabndt(&self) -> IdmabndtR {
        IdmabndtR::new(((self.bits >> 5) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmabndt(&mut self) -> IdmabndtW<IdmabsizerSpec> {
        IdmabndtW::new(self, 5)
    }
}
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`idmabsizer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabsizer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdmabsizerSpec;
impl crate::RegisterSpec for IdmabsizerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmabsizer::R`](R) reader structure"]
impl crate::Readable for IdmabsizerSpec {}
#[doc = "`write(|w| ..)` method takes [`idmabsizer::W`](W) writer structure"]
impl crate::Writable for IdmabsizerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDMABSIZER to value 0"]
impl crate::Resettable for IdmabsizerSpec {}
