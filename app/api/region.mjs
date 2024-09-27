export function GET(request) {
  return new Response(`Region: ${process.env.VERCEL_REGION}`);
}
