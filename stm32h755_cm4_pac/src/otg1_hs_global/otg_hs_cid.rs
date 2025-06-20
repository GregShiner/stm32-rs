#[doc = "Register `OTG_HS_CID` reader"]
pub type R = crate::R<OtgHsCidSpec>;
#[doc = "Register `OTG_HS_CID` writer"]
pub type W = crate::W<OtgHsCidSpec>;
#[doc = "Field `PRODUCT_ID` reader - Product ID field"]
pub type ProductIdR = crate::FieldReader<u32>;
#[doc = "Field `PRODUCT_ID` writer - Product ID field"]
pub type ProductIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&self) -> ProductIdR {
        ProductIdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&mut self) -> ProductIdW<OtgHsCidSpec> {
        ProductIdW::new(self, 0)
    }
}
#[doc = "OTG_HS core ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_cid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_cid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsCidSpec;
impl crate::RegisterSpec for OtgHsCidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_cid::R`](R) reader structure"]
impl crate::Readable for OtgHsCidSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_cid::W`](W) writer structure"]
impl crate::Writable for OtgHsCidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_CID to value 0x1200"]
impl crate::Resettable for OtgHsCidSpec {
    const RESET_VALUE: u32 = 0x1200;
}
