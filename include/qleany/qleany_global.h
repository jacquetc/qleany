#pragma once

#include <QtCore/QtGlobal>

#if defined(QLEANY_LIBRARY)
#define QLEANY_EXPORT Q_DECL_EXPORT
#else // if defined(QLEANY_LIBRARY)
#define QLEANY_EXPORT Q_DECL_IMPORT
#endif // if defined(QLEANY_LIBRARY)
