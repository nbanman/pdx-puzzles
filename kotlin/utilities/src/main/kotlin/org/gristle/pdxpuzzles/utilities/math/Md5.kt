package org.gristle.pdxpuzzles.utilities.math

import java.security.MessageDigest

object Md5 {
    private val mDigest = MessageDigest.getInstance("MD5")!! // getInstance annotated @NotNull
    private const val HEXCHARS = "0123456789abcdef"

    fun getDigest(s: String): ByteArray = mDigest.apply { update(s.toByteArray()) }.digest()

    fun ByteArray.toHex(): String {
        val r = StringBuilder(size * 2)
        forEach { b ->
            val i = b.toInt()
            r.append(HEXCHARS[i shr 4 and 0xF])
            r.append(HEXCHARS[i and 0xF])
        }
        return r.toString()
    }

    fun hashOf(s: String) = getDigest(s).toHex()
}

fun String.md5() = Md5.hashOf(this)
