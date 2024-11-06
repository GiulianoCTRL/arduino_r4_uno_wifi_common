#[doc = "Register `SRAMPRCR` reader"]
pub type R = crate::R<SRAMPRCR_SPEC>;
#[doc = "Register `SRAMPRCR` writer"]
pub type W = crate::W<SRAMPRCR_SPEC>;
#[doc = "Field `SRAMPRCR` reader - Register Write Control"]
pub type SRAMPRCR_R = crate::BitReader<SRAMPRCR_A>;
#[doc = "Register Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMPRCR_A {
    #[doc = "0: Disable writes to protected registers"]
    _0 = 0,
    #[doc = "1: Enable writes to protected registers."]
    _1 = 1,
}
impl From<SRAMPRCR_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMPRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAMPRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAMPRCR_A {
        match self.bits {
            false => SRAMPRCR_A::_0,
            true => SRAMPRCR_A::_1,
        }
    }
    #[doc = "Disable writes to protected registers"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAMPRCR_A::_0
    }
    #[doc = "Enable writes to protected registers."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAMPRCR_A::_1
    }
}
#[doc = "Field `SRAMPRCR` writer - Register Write Control"]
pub type SRAMPRCR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRAMPRCR_A>;
impl<'a, REG, const O: u8> SRAMPRCR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable writes to protected registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMPRCR_A::_0)
    }
    #[doc = "Enable writes to protected registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMPRCR_A::_1)
    }
}
#[doc = "Write Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KW_AW {
    #[doc = "120: Writing to the RAMPRCR bit is valid, when the KEY bits are written 1111000b."]
    _1111000 = 120,
}
impl From<KW_AW> for u8 {
    #[inline(always)]
    fn from(variant: KW_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KW_AW {
    type Ux = u8;
}
#[doc = "Field `KW` writer - Write Key Code"]
pub type KW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O, KW_AW>;
impl<'a, REG, const O: u8> KW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing to the RAMPRCR bit is valid, when the KEY bits are written 1111000b."]
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut crate::W<REG> {
        self.variant(KW_AW::_1111000)
    }
}
impl R {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    pub fn sramprcr(&self) -> SRAMPRCR_R {
        SRAMPRCR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn sramprcr(&mut self) -> SRAMPRCR_W<SRAMPRCR_SPEC, 0> {
        SRAMPRCR_W::new(self)
    }
    #[doc = "Bits 1:7 - Write Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn kw(&mut self) -> KW_W<SRAMPRCR_SPEC, 1> {
        KW_W::new(self)
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
#[doc = "SRAM Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sramprcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sramprcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAMPRCR_SPEC;
impl crate::RegisterSpec for SRAMPRCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sramprcr::R`](R) reader structure"]
impl crate::Readable for SRAMPRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sramprcr::W`](W) writer structure"]
impl crate::Writable for SRAMPRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMPRCR to value 0"]
impl crate::Resettable for SRAMPRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
