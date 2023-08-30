#include "qleany/tools/undo_redo/undo_redo_scopes.h"
#include <QRegularExpression>

Qleany::Tools::UndoRedo::Scopes::Scopes(const QStringList &scopeList)
{

    // Initialize the bit flags to 0x01, which is equivalent to 1
    int n = 0x01;

    // Loop through the list of scopes
    for (const auto &scope : scopeList)
    {
        // If the scope is "all", and exit the loop
        if (scope.toLower() == "all")
        {
            qFatal("do not add All to scopes");
        }

        // Add the scope to the list and map its flag value
        m_scopeList.append(scope);
        m_scopeHash.insert(scope, n);
        m_scopeMap.insert(n, scope);
        m_flags += n;

        // Increment the bit flag to the next power of 2
        n <<= 1;
    }
}

Qleany::Tools::UndoRedo::Scopes::Scopes(const QString &scopeList)
    : Scopes(scopeList.split(QRegularExpression("[\\s|,]+"), Qt::SkipEmptyParts))
{
}

Qleany::Tools::UndoRedo::Scope Qleany::Tools::UndoRedo::Scopes::createScopeFromString(const QString &scopeString)
{
    static auto expr = QRegularExpression("[\\s|,]+");
    return createScopeFromString(scopeString.split(expr, Qt::SkipEmptyParts));
}
