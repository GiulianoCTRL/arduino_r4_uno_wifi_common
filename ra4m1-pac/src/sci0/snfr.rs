#[doc = "Register `SNFR` reader"]
pub type R = crate::R<SNFR_SPEC>;
#[doc = "Register `SNFR` writer"]
pub type W = crate::W<SNFR_SPEC>;
#[doc = "Field `NFCS` reader - Noise Filter Clock Select"]
pub type NFCS_R = crate::FieldReader<NFCS_A>;
#[doc = "Noise Filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    #[doc = "0: The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    _000 = 0,
    #[doc = "1: The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    _001 = 1,
    #[doc = "2: The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    _010 = 2,
    #[doc = "3: The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    _011 = 3,
    #[doc = "4: The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    _100 = 4,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NFCS_A {
    type Ux = u8;
}
impl NFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NFCS_A> {
        match self.bits {
            0 => Some(NFCS_A::_000),
            1 => Some(NFCS_A::_001),
            2 => Some(NFCS_A::_010),
            3 => Some(NFCS_A::_011),
            4 => Some(NFCS_A::_100),
            _ => None,
        }
    }
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == NFCS_A::_000
    }
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == NFCS_A::_001
    }
    #[doc = "The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == NFCS_A::_010
    }
    #[doc = "The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == NFCS_A::_011
    }
    #[doc = "The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == NFCS_A::_100
    }
}
#[doc = "Field `NFCS` writer - Noise Filter Clock Select"]
pub type NFCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, NFCS_A>;
impl<'a, REG, const O: u8> NFCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_000)
    }
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_001)
    }
    #[doc = "The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_010)
    }
    #[doc = "The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_011)
    }
    #[doc = "The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Noise Filter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcs(&mut self) -> NFCS_W<SNFR_SPEC, 0> {
        NFCS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Noise Filter Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNFR_SPEC;
impl crate::RegisterSpec for SNFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snfr::R`](R) reader structure"]
impl crate::Readable for SNFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`snfr::W`](W) writer structure"]
impl crate::Writable for SNFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNFR to value 0"]
impl crate::Resettable for SNFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
