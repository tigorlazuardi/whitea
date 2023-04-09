import 'package:collapsible_sidebar/collapsible_sidebar.dart';
import 'package:flutter/material.dart';

class CustomSidebar extends StatelessWidget {
  final Widget child;
  final bool enable;
  final bool closeOnSelection;

  const CustomSidebar({
    super.key,
    required this.child,
    this.enable = true,
    this.closeOnSelection = false,
  });

  Function onPressed(BuildContext context) {
    return () {
      if (closeOnSelection) {
        Navigator.of(context).pop();
      }
    };
  }

  List<CollapsibleItem> _items(BuildContext context) {
    return [
      CollapsibleItem(
        text: 'Download',
        icon: Icons.download,
        onPressed: onPressed(context),
        isSelected: true,
      ),
      CollapsibleItem(
        text: 'Search',
        icon: Icons.search,
        onPressed: onPressed(context),
      ),
      CollapsibleItem(
        text: 'Settings',
        icon: Icons.settings,
        onPressed: onPressed(context),
      ),
    ];
  }

  @override
  Widget build(BuildContext context) {
    if (!enable) {
      return child;
    }
    final theme = Theme.of(context);
    return CollapsibleSidebar(
      items: _items(context),
      body: child,
      isCollapsed: false,
      showTitle: false,
      showToggleButton: true,
      screenPadding: 0,
      borderRadius: 0,
      backgroundColor: theme.primaryColor,
      selectedIconColor: theme.colorScheme.secondary,
      selectedTextColor: theme.colorScheme.secondary,
      selectedIconBox: Colors.white,
      unselectedIconColor: Colors.white70,
      unselectedTextColor: Colors.white70,
      sidebarBoxShadow: [
        BoxShadow(
          color: theme.shadowColor.withAlpha(100),
          blurRadius: 10,
          spreadRadius: 0.01,
          offset: const Offset(3, 3),
        )
      ],
    );
  }
}
