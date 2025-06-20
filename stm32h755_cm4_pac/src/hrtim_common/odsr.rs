#[doc = "Register `ODSR` reader"]
pub type R = crate::R<OdsrSpec>;
#[doc = "Field `TA1ODS` reader - Timer A Output 1 disable status"]
pub type Ta1odsR = crate::BitReader;
#[doc = "Field `TA2ODS` reader - Timer A Output 2 disable status"]
pub type Ta2odsR = crate::BitReader;
#[doc = "Field `TB1ODS` reader - Timer B Output 1 disable status"]
pub type Tb1odsR = crate::BitReader;
#[doc = "Field `TB2ODS` reader - Timer B Output 2 disable status"]
pub type Tb2odsR = crate::BitReader;
#[doc = "Field `TC1ODS` reader - Timer C Output 1 disable status"]
pub type Tc1odsR = crate::BitReader;
#[doc = "Field `TC2ODS` reader - Timer C Output 2 disable status"]
pub type Tc2odsR = crate::BitReader;
#[doc = "Field `TD1ODS` reader - Timer D Output 1 disable status"]
pub type Td1odsR = crate::BitReader;
#[doc = "Field `TD2ODS` reader - Timer D Output 2 disable status"]
pub type Td2odsR = crate::BitReader;
#[doc = "Field `TE1ODS` reader - Timer E Output 1 disable status"]
pub type Te1odsR = crate::BitReader;
#[doc = "Field `TE2ODS` reader - Timer E Output 2 disable status"]
pub type Te2odsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer A Output 1 disable status"]
    #[inline(always)]
    pub fn ta1ods(&self) -> Ta1odsR {
        Ta1odsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 disable status"]
    #[inline(always)]
    pub fn ta2ods(&self) -> Ta2odsR {
        Ta2odsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Output 1 disable status"]
    #[inline(always)]
    pub fn tb1ods(&self) -> Tb1odsR {
        Tb1odsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer B Output 2 disable status"]
    #[inline(always)]
    pub fn tb2ods(&self) -> Tb2odsR {
        Tb2odsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer C Output 1 disable status"]
    #[inline(always)]
    pub fn tc1ods(&self) -> Tc1odsR {
        Tc1odsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer C Output 2 disable status"]
    #[inline(always)]
    pub fn tc2ods(&self) -> Tc2odsR {
        Tc2odsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer D Output 1 disable status"]
    #[inline(always)]
    pub fn td1ods(&self) -> Td1odsR {
        Td1odsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer D Output 2 disable status"]
    #[inline(always)]
    pub fn td2ods(&self) -> Td2odsR {
        Td2odsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer E Output 1 disable status"]
    #[inline(always)]
    pub fn te1ods(&self) -> Te1odsR {
        Te1odsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer E Output 2 disable status"]
    #[inline(always)]
    pub fn te2ods(&self) -> Te2odsR {
        Te2odsR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Output Disable Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`odsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdsrSpec;
impl crate::RegisterSpec for OdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odsr::R`](R) reader structure"]
impl crate::Readable for OdsrSpec {}
#[doc = "`reset()` method sets ODSR to value 0"]
impl crate::Resettable for OdsrSpec {}
