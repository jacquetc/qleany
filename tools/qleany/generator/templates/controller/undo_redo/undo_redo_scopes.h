// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include <QHash>
#include <QMap>
#include <QObject>
#include <QString>

#include "{{ export_header_file }}"

namespace {{ application_cpp_domain_name }}::Controller::UndoRedo
{
using ScopeFlag = int;

// Scope struc to to hold the command scope
struct {{ export }} Scope

{
    Q_GADGET

  public:
    int scope() const
    {
        return m_scope;
    }

    void setScope(int newScope)
    {
        m_scope = newScope;

        QList<ScopeFlag> flagList;
        for (int i = 0; i <= 30; i++)
        {
            ScopeFlag flag = 1 << i;
            if ((m_scope & flag) != 0)
            {
                flagList.append(flag);
            }
        }
        m_flags = flagList;
    }
    int m_scope;

    bool hasScopeFlag(ScopeFlag scopeFlag) const
    {
        return (m_scope & scopeFlag) != 0;
    }

    // Returns the list of flag values
    QList<ScopeFlag> flags() const
    {
        return m_flags;
    }

    QList<ScopeFlag> m_flags;
};

// Define the operator== function for Scope
inline bool operator==(const {{ application_cpp_domain_name }}::Controller::UndoRedo::Scope &s1, const {{ application_cpp_domain_name }}::Controller::UndoRedo::Scope &s2)
{
    return s1.scope() == s2.scope();
}

// Define the qHash function for Scope
inline uint qHash(const {{ application_cpp_domain_name }}::Controller::UndoRedo::Scope &scope, uint seed = 0)
{
    return scope.scope();
}

// Scopes class to manage multiple scopes for undo-redo commands
class {{ export }} Scopes
{
    Q_GADGET

  public:
    // Constructor taking a QStringList of scopes
    Scopes(const QStringList &scopeList);

    // Constructor taking a comma-separated string of scopes
    Scopes(const QString &scopeList);

    // Returns the number of scopes in the list
    int count() const
    {
        return m_scopeList.count();
    }

    // Returns a QStringList of the scopes
    QStringList scopeList() const
    {
        return m_scopeList;
    }

    // Returns the flag value for a given scope
    ScopeFlag flags(const QString &scope) const
    {
        return m_scopeHash.value(scope, 0);
    }

    // Returns the list of flag values for all scopes
    QList<ScopeFlag> flags() const
    {
        return m_scopeMap.keys();
    }

    // Returns the "all" flag;
    ScopeFlag flagForAll() const
    {
        return 0xFFFFFFF;
    }

    // Returns true if the given scope is in the list
    bool hasScope(const QString &scope) const
    {
        if (scope.toLower() == QString::fromLatin1("all"))
        {
            return true;
        }
        return m_scopeHash.contains(scope);
    }

    // Returns true if the given scope is in the list
    bool hasScope(const Scope &scope) const
    {
        if (scope.scope() == 0xFFFFFFF)
        {
            return true;
        }

        return (m_flags & scope.scope()) != 0;
    }

    Scope createScopeFromString(const QStringList &scopeStringList)
    {
        int n = 0x00;

        for (const auto &scope : scopeStringList)
        {
            int scopeFlag = 0;
            if (scope == QString::fromLatin1("all"))
            {
                n = 0xFFFFFFF;
                break;
            }
            else
                scopeFlag = m_scopeHash.value(scope, 0);
            n += scopeFlag;

            if (scopeFlag == 0)
            {
                QString fatal = QString::fromLatin1("At Scopes::createScopeFromString, unknown scope : %1").arg(scope);
                qFatal("%s", fatal.toStdString().c_str());
            }
        }

        Scope scope;
        scope.setScope(n);

        return scope;
    }
    Scope createScopeFromString(const QString &scopeString);

  private:
    // The combined flag value for all scopes
    int m_flags = 0;

    // The list of scopes
    QStringList m_scopeList;

    // The map of scope names to flag values
    QHash<QString, int> m_scopeHash;
    QMap<int, QString> m_scopeMap;
};

} // namespace {{ application_cpp_domain_name }}::Controller::UndoRedo
Q_DECLARE_METATYPE({{ application_cpp_domain_name }}::Controller::UndoRedo::Scope)
