#[doc = "Register `ECCPRCR2` reader"]
pub type R = crate::R<ECCPRCR2_SPEC>;
#[doc = "Register `ECCPRCR2` writer"]
pub type W = crate::W<ECCPRCR2_SPEC>;
#[doc = "Field `ECCPRCR2` reader - Register Write Control"]
pub type ECCPRCR2_R = crate::BitReader<ECCPRCR2_A>;
#[doc = "Register Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCPRCR2_A {
    #[doc = "0: Disable writes to the protected registers"]
    _0 = 0,
    #[doc = "1: Enable writes to the protected registers."]
    _1 = 1,
}
impl From<ECCPRCR2_A> for bool {
    #[inline(always)]
    fn from(variant: ECCPRCR2_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCPRCR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCPRCR2_A {
        match self.bits {
            false => ECCPRCR2_A::_0,
            true => ECCPRCR2_A::_1,
        }
    }
    #[doc = "Disable writes to the protected registers"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECCPRCR2_A::_0
    }
    #[doc = "Enable writes to the protected registers."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECCPRCR2_A::_1
    }
}
#[doc = "Field `ECCPRCR2` writer - Register Write Control"]
pub type ECCPRCR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ECCPRCR2_A>;
impl<'a, REG, const O: u8> ECCPRCR2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable writes to the protected registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPRCR2_A::_0)
    }
    #[doc = "Enable writes to the protected registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPRCR2_A::_1)
    }
}
#[doc = "Write Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KW2_AW {
    #[doc = "120: These bits enable or disable writes to the ECCPRCR2 bit.."]
    _1111000 = 120,
}
impl From<KW2_AW> for u8 {
    #[inline(always)]
    fn from(variant: KW2_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KW2_AW {
    type Ux = u8;
}
#[doc = "Field `KW2` writer - Write Key Code"]
pub type KW2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O, KW2_AW>;
impl<'a, REG, const O: u8> KW2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "These bits enable or disable writes to the ECCPRCR2 bit.."]
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut crate::W<REG> {
        self.variant(KW2_AW::_1111000)
    }
}
impl R {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    pub fn eccprcr2(&self) -> ECCPRCR2_R {
        ECCPRCR2_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn eccprcr2(&mut self) -> ECCPRCR2_W<ECCPRCR2_SPEC, 0> {
        ECCPRCR2_W::new(self)
    }
    #[doc = "Bits 1:7 - Write Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn kw2(&mut self) -> KW2_W<ECCPRCR2_SPEC, 1> {
        KW2_W::new(self)
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
#[doc = "ECC Protection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccprcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccprcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCPRCR2_SPEC;
impl crate::RegisterSpec for ECCPRCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eccprcr2::R`](R) reader structure"]
impl crate::Readable for ECCPRCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccprcr2::W`](W) writer structure"]
impl crate::Writable for ECCPRCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCPRCR2 to value 0"]
impl crate::Resettable for ECCPRCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
