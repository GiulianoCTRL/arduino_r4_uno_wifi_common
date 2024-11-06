#[doc = "Register `GTWP` reader"]
pub type R = crate::R<GTWP_SPEC>;
#[doc = "Register `GTWP` writer"]
pub type W = crate::W<GTWP_SPEC>;
#[doc = "Field `WP` reader - Register Write Disable"]
pub type WP_R = crate::BitReader<WP_A>;
#[doc = "Register Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    #[doc = "0: Write to the register is enabled"]
    _0 = 0,
    #[doc = "1: Write to the register is disabled"]
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    #[doc = "Write to the register is enabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    #[doc = "Write to the register is disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
#[doc = "Field `WP` writer - Register Write Disable"]
pub type WP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WP_A>;
impl<'a, REG, const O: u8> WP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write to the register is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WP_A::_0)
    }
    #[doc = "Write to the register is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_A::_1)
    }
}
#[doc = "GTWP Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRKEY_AW {
    #[doc = "165: Written to these bits, the WP bits write is permitted."]
    _0X_A5 = 165,
}
impl From<PRKEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: PRKEY_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRKEY_AW {
    type Ux = u8;
}
#[doc = "Field `PRKEY` writer - GTWP Key Code"]
pub type PRKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, PRKEY_AW>;
impl<'a, REG, const O: u8> PRKEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Written to these bits, the WP bits write is permitted."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(PRKEY_AW::_0X_A5)
    }
}
impl R {
    #[doc = "Bit 0 - Register Write Disable"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<GTWP_SPEC, 0> {
        WP_W::new(self)
    }
    #[doc = "Bits 8:15 - GTWP Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn prkey(&mut self) -> PRKEY_W<GTWP_SPEC, 8> {
        PRKEY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "General PWM Timer Write-Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtwp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtwp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTWP_SPEC;
impl crate::RegisterSpec for GTWP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtwp::R`](R) reader structure"]
impl crate::Readable for GTWP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtwp::W`](W) writer structure"]
impl crate::Writable for GTWP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTWP to value 0"]
impl crate::Resettable for GTWP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
