/** @vitest-environment jsdom */

import React from "react";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import LeaderboardClient from "@/app/(main)/leaderboard/LeaderboardClient";

vi.mock("next/link", () => ({
  default: ({ children, href, ...props }: React.AnchorHTMLAttributes<HTMLAnchorElement>) => (
    <a href={typeof href === "string" ? href : ""} {...props}>{children}</a>
  ),
}));

vi.mock("next/image", () => ({
  default: ({ alt = "", ...props }: React.ImgHTMLAttributes<HTMLImageElement>) => (
    // eslint-disable-next-line @next/next/no-img-element
    <img alt={alt} {...props} />
  ),
}));

vi.mock("nextjs-toploader/app", () => ({
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("next/navigation", () => ({
  usePathname: () => "/leaderboard",
  useSearchParams: () => new URLSearchParams(),
}));

vi.mock("@/lib/useSettings", () => ({
  useSettings: () => ({ leaderboardSortBy: "tokens", setLeaderboardSort: vi.fn(), mounted: false }),
}));

const initialData = {
  users: [],
  pagination: { page: 1, limit: 50, totalUsers: 0, totalPages: 0, hasNext: false, hasPrev: false },
  stats: { totalTokens: 0, totalCost: 0, uniqueUsers: 0 },
  period: "all" as const,
  sortBy: "tokens" as const,
};

const currentUser = {
  id: "user-1",
  username: "new-user",
  displayName: "New User",
  avatarUrl: null,
};

function jsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}

function renderLeaderboard(status: number, body: unknown = {}) {
  vi.stubGlobal("fetch", vi.fn(() => Promise.resolve(jsonResponse(body, status))));
  render(<LeaderboardClient initialData={initialData} currentUser={currentUser} initialSortBy="tokens" />);
}

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

describe("leaderboard user rank", () => {
  it("shows a not-ranked state when the user rank endpoint returns 404", async () => {
    renderLeaderboard(404);

    expect(await screen.findByText(/You're not on the leaderboard yet/)).toBeTruthy();
    expect(screen.queryByText("Unable to load your ranking. Please refresh the page.")).toBeNull();
    expect(screen.getByRole("link", { name: "Join the Leaderboard" }).getAttribute("href")).toBe("#join-leaderboard");

    fireEvent.click(screen.getByRole("button", { name: "This month" }));
    expect(await screen.findByText("No usage submitted for this period yet.")).toBeTruthy();
  });

  it("shows the ranking error when the user rank endpoint returns 500", async () => {
    renderLeaderboard(500);

    expect(await screen.findByText("Unable to load your ranking. Please refresh the page.")).toBeTruthy();
  });

  it("shows neither banner when the user rank endpoint returns a rank", async () => {
    renderLeaderboard(200, {
      rank: 1,
      userId: currentUser.id,
      username: currentUser.username,
      displayName: currentUser.displayName,
      avatarUrl: null,
      totalTokens: 1,
      totalCost: 0,
    });

    await screen.findByText("New User");
    expect(screen.queryByText(/You're not on the leaderboard yet/)).toBeNull();
    expect(screen.queryByText("Unable to load your ranking. Please refresh the page.")).toBeNull();
  });
});
