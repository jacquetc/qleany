// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "result.h"
#include "undo_redo_scopes.h"
#include <QDateTime>
#include <QFutureWatcher>
#include <QObject>
#include <QPromise>
#include <QTimer>
#include "{{ export_header_file }}"

using namespace {{ application_cpp_domain_name }};

namespace {{ application_cpp_domain_name }}::Controller::UndoRedo
{
/*!
 * \ingroup UndoRedo
 * \brief A base class for implementing undo and redo commands.
 * Represents a base class for undo-redo commands in the application. Derived
 * classes should implement undo() and redo()
 * methods to define the behavior of the command during undo and redo
 * operations.
 */
class {{ export }} UndoRedoCommand : public QObject
{
    Q_OBJECT

  public:
    enum Status
    {
        Waiting,
        Running,
        Finished,
    };
    Q_ENUM(Status)

    UndoRedoCommand(const QString &text);

    void setUndoFunction(const std::function<Result<void>()> &function);
    void setRedoFunction(const std::function<void(QPromise<Result<void>> &promise)> &function);
    void setMergeWithFunction(const std::function<bool(const UndoRedoCommand *other)> &function);

    void asyncUndo();

    void asyncRedo();

    bool isRunning() const;
    bool isWaiting() const;
    bool isFinished() const;

    Scope scope() const;

    void setScope(Scope newScope);

    QString text() const;

    void setText(const QString &newText);

    bool obsolete() const;

    void setObsolete(bool newObsolete);

    virtual bool mergeWith(const UndoRedoCommand *other);

    bool isSystem() const;
    void setIsSystem(bool newIsSystem);

    QUuid stackId() const;
    void setStackId(const QUuid &newStackId);

    bool isAlterCommand() const;
    bool isQueryCommand() const;

    enum Type
    {
        AlterCommand,
        QueryCommand
    };
    Q_ENUM(Type)

    void setProgressMinimumDuration(int minimumDuration);
    int progressMinimumDuration() const;

    Type type() const;

  protected:
    void setType(Type newType);

  Q_SIGNALS:

    void finished(bool isSuccessful);

    /*!
     * \brief A signal that is emitted when a command results in an error.
     * actions.
     */
    void errorSent(Error error);
    void warningSent(Error error);

    // progress Q_SIGNALS
    void progressStarted();
    void progressFinished();
    void progressRangeChanged(int minimum, int maximum);
    void progressTextChanged(const QString &progressText);
    void progressValueChanged(int progressValue);

    void undoing({{ application_cpp_domain_name }}::Controller::UndoRedo::Scope scope, bool active);
    void redoing({{ application_cpp_domain_name }}::Controller::UndoRedo::Scope scope, bool active);

  private Q_SLOTS:

    void onFinished();
    void progressTimerTimeout();

  private:
    std::function<Result<void>()> m_undoFunction;
    std::function<void(QPromise<Result<void>> &promise)> m_redoFunction;
    std::function<bool(const UndoRedoCommand *other)> m_mergeWithFunction;

    QFutureWatcher<Result<void>> *m_watcher;
    bool m_obsolete;         /*!< A boolean representing the obsolete state of the
                                command. */
    bool m_isSystem = false; /*!< A boolean representing the command is a system command
                                (true) or a user command (false). */
    QString m_text;          /*!< A QString representing the text description of the
                                command. */
    Scope m_scope;           /*!< The command's scope as an UndoRedoCommand::Scope
                                enumeration value. */
    Status m_status;         /*!< An enum representing the state of the command. */
    QUuid m_stackId;         /*!< A QUuid representing the id of the stack the command
                                is in. */
    Type m_type = AlterCommand;
    int m_progressMinimumDuration = 500;
    QDateTime m_startTime;
    QDateTime m_finishTime;
    QTimer *m_progressTimer = nullptr;
};
} // namespace {{ application_cpp_domain_name }}::Controller::UndoRedo
