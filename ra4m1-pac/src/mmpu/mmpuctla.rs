#[doc = "Register `MMPUCTLA` reader"]
pub type R = crate::R<MMPUCTLA_SPEC>;
#[doc = "Register `MMPUCTLA` writer"]
pub type W = crate::W<MMPUCTLA_SPEC>;
#[doc = "Field `ENABLE` reader - Master Group enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Master Group enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Master Group A disabled"]
    _0 = 0,
    #[doc = "1: Master Group A enabled."]
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    #[doc = "Master Group A disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    #[doc = "Master Group A enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
#[doc = "Field `ENABLE` writer - Master Group enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENABLE_A>;
impl<'a, REG, const O: u8> ENABLE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master Group A disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "Master Group A enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_1)
    }
}
#[doc = "Field `OAD` reader - Operation after detection"]
pub type OAD_R = crate::BitReader<OAD_A>;
#[doc = "Operation after detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    #[doc = "0: Non-maskable interrupt."]
    _0 = 0,
    #[doc = "1: Internal reset."]
    _1 = 1,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OAD_A {
        match self.bits {
            false => OAD_A::_0,
            true => OAD_A::_1,
        }
    }
    #[doc = "Non-maskable interrupt."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
    #[doc = "Internal reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
}
#[doc = "Field `OAD` writer - Operation after detection"]
pub type OAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OAD_A>;
impl<'a, REG, const O: u8> OAD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_0)
    }
    #[doc = "Internal reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code These bits are used to enable or disable writing of the OAD and ENABLE bit."]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation after detection"]
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<MMPUCTLA_SPEC, 0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Operation after detection"]
    #[inline(always)]
    #[must_use]
    pub fn oad(&mut self) -> OAD_W<MMPUCTLA_SPEC, 1> {
        OAD_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code These bits are used to enable or disable writing of the OAD and ENABLE bit."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<MMPUCTLA_SPEC, 8> {
        KEY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bus Master MPU Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpuctla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpuctla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMPUCTLA_SPEC;
impl crate::RegisterSpec for MMPUCTLA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mmpuctla::R`](R) reader structure"]
impl crate::Readable for MMPUCTLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmpuctla::W`](W) writer structure"]
impl crate::Writable for MMPUCTLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUCTLA to value 0"]
impl crate::Resettable for MMPUCTLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
