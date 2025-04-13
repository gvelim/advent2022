// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Solutions</li><li class="chapter-item expanded "><a href="day1/index.html"><strong aria-hidden="true">1.</strong> Day 1: Calorie Counting</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day1/problem.html"><strong aria-hidden="true">1.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day1/solution.html"><strong aria-hidden="true">1.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day1/code.html"><strong aria-hidden="true">1.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day2/index.html"><strong aria-hidden="true">2.</strong> Day 2: Rock Paper Scissors</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day2/problem.html"><strong aria-hidden="true">2.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day2/solution.html"><strong aria-hidden="true">2.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day2/code.html"><strong aria-hidden="true">2.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day3/index.html"><strong aria-hidden="true">3.</strong> Day 3: Rucksack Reorganization</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day3/problem.html"><strong aria-hidden="true">3.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day3/solution.html"><strong aria-hidden="true">3.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day3/code.html"><strong aria-hidden="true">3.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day4/index.html"><strong aria-hidden="true">4.</strong> Day 4: Camp Cleanup</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day4/problem.html"><strong aria-hidden="true">4.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day4/solution.html"><strong aria-hidden="true">4.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day4/code.html"><strong aria-hidden="true">4.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day5/index.html"><strong aria-hidden="true">5.</strong> Day 5: Supply Stacks</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day5/problem.html"><strong aria-hidden="true">5.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day5/solution.html"><strong aria-hidden="true">5.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day5/code.html"><strong aria-hidden="true">5.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day6/index.html"><strong aria-hidden="true">6.</strong> Day 6: Tuning Trouble</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day6/problem.html"><strong aria-hidden="true">6.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day6/solution.html"><strong aria-hidden="true">6.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day6/code.html"><strong aria-hidden="true">6.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day7/index.html"><strong aria-hidden="true">7.</strong> Day 7: No Space Left On Device</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day7/problem.html"><strong aria-hidden="true">7.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day7/solution.html"><strong aria-hidden="true">7.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day7/code.html"><strong aria-hidden="true">7.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day8/index.html"><strong aria-hidden="true">8.</strong> Day 8: Treetop Tree House</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day8/problem.html"><strong aria-hidden="true">8.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day8/solution.html"><strong aria-hidden="true">8.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day8/code.html"><strong aria-hidden="true">8.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day9/index.html"><strong aria-hidden="true">9.</strong> Day 9: Rope Bridge</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day9/problem.html"><strong aria-hidden="true">9.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day9/solution.html"><strong aria-hidden="true">9.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day9/code.html"><strong aria-hidden="true">9.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day10/index.html"><strong aria-hidden="true">10.</strong> Day 10: Cathode-Ray Tube</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day10/problem.html"><strong aria-hidden="true">10.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day10/solution.html"><strong aria-hidden="true">10.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day10/code.html"><strong aria-hidden="true">10.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day11/index.html"><strong aria-hidden="true">11.</strong> Day 11: Monkey in the Middle</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day11/problem.html"><strong aria-hidden="true">11.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day11/solution.html"><strong aria-hidden="true">11.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day11/code.html"><strong aria-hidden="true">11.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day12/index.html"><strong aria-hidden="true">12.</strong> Day 12: Hill Climbing Algorithm</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day12/problem.html"><strong aria-hidden="true">12.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day12/solution.html"><strong aria-hidden="true">12.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day12/code.html"><strong aria-hidden="true">12.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day13/index.html"><strong aria-hidden="true">13.</strong> Day 13: Distress Signal</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day13/problem.html"><strong aria-hidden="true">13.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day13/solution.html"><strong aria-hidden="true">13.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day13/code.html"><strong aria-hidden="true">13.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day14/index.html"><strong aria-hidden="true">14.</strong> Day 14: Regolith Reservoir</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day14/problem.html"><strong aria-hidden="true">14.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day14/solution.html"><strong aria-hidden="true">14.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day14/code.html"><strong aria-hidden="true">14.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day15/index.html"><strong aria-hidden="true">15.</strong> Day 15: Beacon Exclusion Zone</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day15/problem.html"><strong aria-hidden="true">15.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day15/solution.html"><strong aria-hidden="true">15.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day15/code.html"><strong aria-hidden="true">15.3.</strong> Code</a></li></ol></li><li class="chapter-item expanded "><a href="day16/index.html"><strong aria-hidden="true">16.</strong> Day 16: Proboscidea Volcanium</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="day16/problem.html"><strong aria-hidden="true">16.1.</strong> Problem Description</a></li><li class="chapter-item expanded "><a href="day16/solution.html"><strong aria-hidden="true">16.2.</strong> Solution Explanation</a></li><li class="chapter-item expanded "><a href="day16/code.html"><strong aria-hidden="true">16.3.</strong> Code</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
