#[doc = "Register `RESP3R` reader"]
pub type R = crate::R<Resp3rSpec>;
#[doc = "Field `CARDSTATUS3` reader - see Table404."]
pub type Cardstatus3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table404."]
    #[inline(always)]
    pub fn cardstatus3(&self) -> Cardstatus3R {
        Cardstatus3R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp3rSpec;
impl crate::RegisterSpec for Resp3rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3r::R`](R) reader structure"]
impl crate::Readable for Resp3rSpec {}
#[doc = "`reset()` method sets RESP3R to value 0"]
impl crate::Resettable for Resp3rSpec {}
