#[doc = "Register `RESP2R` reader"]
pub type R = crate::R<Resp2rSpec>;
#[doc = "Field `CARDSTATUS2` reader - see Table404."]
pub type Cardstatus2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table404."]
    #[inline(always)]
    pub fn cardstatus2(&self) -> Cardstatus2R {
        Cardstatus2R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp2rSpec;
impl crate::RegisterSpec for Resp2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2r::R`](R) reader structure"]
impl crate::Readable for Resp2rSpec {}
#[doc = "`reset()` method sets RESP2R to value 0"]
impl crate::Resettable for Resp2rSpec {}
