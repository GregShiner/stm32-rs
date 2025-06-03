#[doc = "Register `RESP1R` reader"]
pub type R = crate::R<Resp1rSpec>;
#[doc = "Field `CARDSTATUS1` reader - see Table 432"]
pub type Cardstatus1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 432"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> Cardstatus1R {
        Cardstatus1R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp1rSpec;
impl crate::RegisterSpec for Resp1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp1r::R`](R) reader structure"]
impl crate::Readable for Resp1rSpec {}
#[doc = "`reset()` method sets RESP1R to value 0"]
impl crate::Resettable for Resp1rSpec {}
