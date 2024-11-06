#[doc = "Register `MSPMPUPT` reader"]
pub type R = crate::R<MSPMPUPT_SPEC>;
#[doc = "Register `MSPMPUPT` writer"]
pub type W = crate::W<MSPMPUPT_SPEC>;
#[doc = "Field `PROTECT` reader - Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
pub type PROTECT_R = crate::BitReader<PROTECT_A>;
#[doc = "Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROTECT_A {
    #[doc = "0: Stack Pointer Monitor register writing is possible."]
    _0 = 0,
    #[doc = "1: Stack Pointer Monitor register writing is protected."]
    _1 = 1,
}
impl From<PROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: PROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PROTECT_A {
        match self.bits {
            false => PROTECT_A::_0,
            true => PROTECT_A::_1,
        }
    }
    #[doc = "Stack Pointer Monitor register writing is possible."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROTECT_A::_0
    }
    #[doc = "Stack Pointer Monitor register writing is protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROTECT_A::_1
    }
}
#[doc = "Field `PROTECT` writer - Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
pub type PROTECT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PROTECT_A>;
impl<'a, REG, const O: u8> PROTECT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stack Pointer Monitor register writing is possible."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PROTECT_A::_0)
    }
    #[doc = "Stack Pointer Monitor register writing is protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PROTECT_A::_1)
    }
}
#[doc = "Write Keyword The data written to these bits are not stored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "165: Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    _0X_A5 = 165,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_AW {
    type Ux = u8;
}
#[doc = "Field `KEY` writer - Write Keyword The data written to these bits are not stored."]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, KEY_AW>;
impl<'a, REG, const O: u8> KEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_AW::_0X_A5)
    }
}
impl R {
    #[doc = "Bit 0 - Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> PROTECT_W<MSPMPUPT_SPEC, 0> {
        PROTECT_W::new(self)
    }
    #[doc = "Bits 8:15 - Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<MSPMPUPT_SPEC, 8> {
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
#[doc = "Stack Pointer Monitor Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpupt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpupt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSPMPUPT_SPEC;
impl crate::RegisterSpec for MSPMPUPT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mspmpupt::R`](R) reader structure"]
impl crate::Readable for MSPMPUPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mspmpupt::W`](W) writer structure"]
impl crate::Writable for MSPMPUPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPMPUPT to value 0"]
impl crate::Resettable for MSPMPUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
