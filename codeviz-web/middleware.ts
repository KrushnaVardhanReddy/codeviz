import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'
import { auth } from './auth'

export default auth((req) => {
  if (!req.auth && req.nextUrl.pathname.startsWith("/app")) {
    const newUrl = new URL("/api/auth/signin", req.nextUrl.origin)
    return NextResponse.redirect(newUrl)
  }
})

export const config = {
  matcher: '/app/:path*',
}
