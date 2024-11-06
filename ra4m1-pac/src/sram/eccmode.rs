#[doc = "Register `ECCMODE` reader"]
pub type R = crate::R<ECCMODE_SPEC>;
#[doc = "Register `ECCMODE` writer"]
pub type W = crate::W<ECCMODE_SPEC>;
#[doc = "Field `ECCMOD` reader - ECC Operating Mode Select"]
pub type ECCMOD_R = crate::FieldReader<ECCMOD_A>;
#[doc = "ECC Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECCMOD_A {
    #[doc = "0: Disable ECC function"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: Enable ECC function without error checking"]
    _10 = 2,
    #[doc = "3: Enable ECC function with error checking"]
    _11 = 3,
}
impl From<ECCMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ECCMOD_A {
    type Ux = u8;
}
impl ECCMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCMOD_A {
        match self.bits {
            0 => ECCMOD_A::_00,
            1 => ECCMOD_A::_01,
            2 => ECCMOD_A::_10,
            3 => ECCMOD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable ECC function"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ECCMOD_A::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ECCMOD_A::_01
    }
    #[doc = "Enable ECC function without error checking"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ECCMOD_A::_10
    }
    #[doc = "Enable ECC function with error checking"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ECCMOD_A::_11
    }
}
#[doc = "Field `ECCMOD` writer - ECC Operating Mode Select"]
pub type ECCMOD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, ECCMOD_A>;
impl<'a, REG, const O: u8> ECCMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable ECC function"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMOD_A::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMOD_A::_01)
    }
    #[doc = "Enable ECC function without error checking"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMOD_A::_10)
    }
    #[doc = "Enable ECC function with error checking"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMOD_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(&self) -> ECCMOD_R {
        ECCMOD_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - ECC Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn eccmod(&mut self) -> ECCMOD_W<ECCMODE_SPEC, 0> {
        ECCMOD_W::new(self)
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
#[doc = "ECC Operating Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCMODE_SPEC;
impl crate::RegisterSpec for ECCMODE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eccmode::R`](R) reader structure"]
impl crate::Readable for ECCMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccmode::W`](W) writer structure"]
impl crate::Writable for ECCMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCMODE to value 0"]
impl crate::Resettable for ECCMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
