// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#include <QCoreApplication>
#include <QDeadlineTimer>
#include <undo_redo/threaded_undo_redo_system.h>

using namespace std::chrono;
using namespace std::chrono_literals;
using namespace {{ application_cpp_domain_name }}::Controller::UndoRedo;

ThreadedUndoRedoSystem *ThreadedUndoRedoSystem::m_instance = nullptr;


/*!
 * \brief Constructs an ThreadedUndoRedoSystem with the specified \a parent.
 */
ThreadedUndoRedoSystem::ThreadedUndoRedoSystem(QObject *parent, const Scopes &scopes) : QObject(parent)
{
    QMutexLocker locker(&m_mutex);

    // singleton
    if (m_instance)
        return;

    m_instance = this;

    // Create a new UndoRedoSystem instance
    m_undoRedoSystemWorker = new UndoRedoSystem(m_undoRedoSystemWorker, scopes);

    // Connect the UndoRedoSystem's stateChanged signal to this class's
    // stateChanged signal
    connect(m_undoRedoSystemWorker, &UndoRedoSystem::stateChanged, this,
            &ThreadedUndoRedoSystem::onUndoRedoSystemStateChanged, Qt::QueuedConnection);
    connect(m_undoRedoSystemWorker, &UndoRedoSystem::warningSent, this, &ThreadedUndoRedoSystem::onWarningSent,
            Qt::QueuedConnection);
    connect(m_undoRedoSystemWorker, &UndoRedoSystem::errorSent, this, &ThreadedUndoRedoSystem::onErrorSent,
            Qt::QueuedConnection);
    connect(m_undoRedoSystemWorker, &UndoRedoSystem::undoing, this, &ThreadedUndoRedoSystem::undoing,
            Qt::QueuedConnection);
    connect(m_undoRedoSystemWorker, &UndoRedoSystem::redoing, this, &ThreadedUndoRedoSystem::redoing,
            Qt::QueuedConnection);

    connect(qApp, &QCoreApplication::aboutToQuit, this, &ThreadedUndoRedoSystem::quitGracefully);

    // Start the thread
    m_undoRedoSystemWorker->start();
}

ThreadedUndoRedoSystem::~ThreadedUndoRedoSystem()
{
    QMutexLocker locker(&m_mutex);

    // Stop the thread
    m_undoRedoSystemWorker->quit();
    m_undoRedoSystemWorker->wait();

    // Delete the UndoRedoSystem instance and the thread
    m_undoRedoSystemWorker->deleteLater();

    // m_undoRedoSystemWorker->deleteLater();
}

/*!
 * \brief Returns the singleton instance of ThreadedUndoRedoSystem.
 * \note If no instance exists, a fatal error occurs.
 */
ThreadedUndoRedoSystem *ThreadedUndoRedoSystem::instance()
{
    if (!m_instance)
        qFatal("No ThreadedUndoRedoSystem instance found");
    return m_instance;
}

/*!
 * \brief Returns true if an undo operation can be performed, otherwise false.
 */
bool ThreadedUndoRedoSystem::canUndo() const
{
    QMutexLocker locker(&m_mutex);
    bool result = false;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "canUndo", Qt::QueuedConnection, Q_RETURN_ARG(bool, result));
    return result;
}

/*!
 * \brief Returns true if a redo operation can be performed, otherwise false.
 */
bool ThreadedUndoRedoSystem::canRedo() const
{
    QMutexLocker locker(&m_mutex);
    bool result = false;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "canRedo", Qt::QueuedConnection, Q_RETURN_ARG(bool, result));
    return result;
}

/*!
 * \brief Performs an undo operation.
 */
void ThreadedUndoRedoSystem::undo()
{
    QMutexLocker locker(&m_mutex);

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "undo", Qt::QueuedConnection);
}

/*!
 * \brief Performs a redo operation.
 */
void ThreadedUndoRedoSystem::redo()
{
    QMutexLocker locker(&m_mutex);

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "redo", Qt::QueuedConnection);
}

/*!
 * \brief Adds a new command to the command history with the specified \a scope.
 */
void ThreadedUndoRedoSystem::push(UndoRedoCommand *command, const QString &commandScope, const QUuid &stackId)
{
    QMutexLocker locker(&m_mutex);

    command->moveToThread(m_undoRedoSystemWorker->thread());
    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "push", Qt::QueuedConnection, Q_ARG(UndoRedoCommand *, command),
                              Q_ARG(QString, commandScope), Q_ARG(QUuid, stackId));
}

/*!
 * \brief Adds a new command to the command history with the specified \a scope.
 */
void ThreadedUndoRedoSystem::push(UndoRedoCommand *command, const QString &commandScope, const QUuid &stackId) const
{
    QMutexLocker locker(&m_mutex);

    command->moveToThread(m_undoRedoSystemWorker->thread());
    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "push", Qt::QueuedConnection, Q_ARG(UndoRedoCommand *, command),
                              Q_ARG(QString, commandScope), Q_ARG(QUuid, stackId));
}

/*!
 * \brief Clears the command history.
 */
void ThreadedUndoRedoSystem::clear()
{
    QMutexLocker locker(&m_mutex);

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "clear", Qt::QueuedConnection);
}

/*!
 * \brief Handles the UndoRedoSystem stateChanged signal and emits the
 * stateChanged signal for this class.
 */
void ThreadedUndoRedoSystem::onUndoRedoSystemStateChanged()
{
    QMutexLocker locker(&m_mutex);

    // Emit the stateChanged signal
    Q_EMIT stateChanged();
}

void ThreadedUndoRedoSystem::onErrorSent(const Error &error)
{
    QMutexLocker locker(&m_mutex);

    // Emit the stateChanged signal
    Q_EMIT errorSent(error);
}

void ThreadedUndoRedoSystem::onWarningSent(const Error &error)
{
    QMutexLocker locker(&m_mutex);

    // Emit the stateChanged signal
    Q_EMIT warningSent(error);
}

/*!
 * \brief Sets the undo limit to \a limit.
 */
void ThreadedUndoRedoSystem::setUndoLimit(int limit)
{
    QMutexLocker locker(&m_mutex);

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "setUndoLimit", Qt::QueuedConnection, Q_ARG(int, limit));
}

/*!
 * \brief Returns the undo limit.
 */
int ThreadedUndoRedoSystem::undoLimit() const
{
    QMutexLocker locker(&m_mutex);
    int result = 0;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "undoLimit", Qt::QueuedConnection, Q_RETURN_ARG(int, result));
    return result;
}

/*!
 * \brief Returns the text of the last undo command.
 */
QString ThreadedUndoRedoSystem::undoText() const
{
    QMutexLocker locker(&m_mutex);
    QString result;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "undoText", Qt::QueuedConnection, Q_RETURN_ARG(QString, result));
    return result;
}

/*!
 * \brief Returns the text of the last redo command.
 */
QString ThreadedUndoRedoSystem::redoText() const
{
    QMutexLocker locker(&m_mutex);
    QString result;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "redoText", Qt::QueuedConnection, Q_RETURN_ARG(QString, result));
    return result;
}

QStringList ThreadedUndoRedoSystem::undoRedoTextList() const
{
    QMutexLocker locker(&m_mutex);
    QStringList result;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "undoRedoTextList", Qt::QueuedConnection,
                              Q_RETURN_ARG(QStringList, result));
    return result;
}

int ThreadedUndoRedoSystem::currentIndex() const
{
    QMutexLocker locker(&m_mutex);
    int result = 0;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "currentIndex", Qt::QueuedConnection, Q_RETURN_ARG(int, result));
    return result;
}

void ThreadedUndoRedoSystem::setCurrentIndex(int index)
{
    QMutexLocker locker(&m_mutex);

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "setCurrentIndex", Qt::QueuedConnection, Q_ARG(int, index));
}

void ThreadedUndoRedoSystem::setActiveStack(const QUuid &stackId)
{
    QMutexLocker locker(&m_mutex);

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "setActiveStack", Qt::QueuedConnection, Q_ARG(QUuid, stackId));
}

QUuid ThreadedUndoRedoSystem::activeStackId() const
{
    QMutexLocker locker(&m_mutex);
    QUuid result;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "activeStackId", Qt::QueuedConnection,
                              Q_RETURN_ARG(QUuid, result));
    return result;
}

void ThreadedUndoRedoSystem::quitGracefully()
{
    QMutexLocker locker(&m_mutex);

    m_undoRedoSystemWorker->requestInterruption();

    // let the thread exit gracefully
    if (m_undoRedoSystemWorker->wait(QDeadlineTimer(60s)))
    {
        m_undoRedoSystemWorker->quit();
        m_undoRedoSystemWorker->wait();
        qInfo() << "UndoRedoSystem thread exited gracefully";
    }
    else
    {
        m_undoRedoSystemWorker->terminate();
        m_undoRedoSystemWorker->wait();
        qCritical() << "UndoRedoSystem thread did not exit gracefully";
    }
}

QStringList ThreadedUndoRedoSystem::queuedCommandTextListByScope(const QString &scopeFlagString) const
{
    QMutexLocker locker(&m_mutex);
    QStringList result;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "queuedCommandTextListByScope", Qt::QueuedConnection,
                              Q_RETURN_ARG(QStringList, result), Q_ARG(QString, scopeFlagString));
    return result;
}

bool ThreadedUndoRedoSystem::isRunning() const
{
    QMutexLocker locker(&m_mutex);
    bool result = false;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "isRunning", Qt::QueuedConnection, Q_RETURN_ARG(bool, result));
    return result;
}

int ThreadedUndoRedoSystem::numberOfCommands() const
{
    QMutexLocker locker(&m_mutex);
    int result = 0;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "numberOfCommands", Qt::QueuedConnection,
                              Q_RETURN_ARG(int, result));
    return result;
}


/*!
 * \brief Creates and returns a redo QAction with the specified \a parent and \a
 * prefix.
 */
QAction *ThreadedUndoRedoSystem::createRedoAction(QObject *parent, const QString &prefix) const
{
    QMutexLocker locker(&m_mutex);
    QAction *action = new QAction(parent);
    bool canRedo = false;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "canRedo", Qt::BlockingQueuedConnection,
                              Q_RETURN_ARG(bool, canRedo));
    QString text;

    if (canRedo)
    {
        QMetaObject::invokeMethod(m_undoRedoSystemWorker, "redoText", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(QString, text));

        if (!text.isEmpty())
        {
            text = prefix.isEmpty() ? tr("Redo %1").arg(text) : prefix.arg(text);
            action->setText(text);
        }
    }

    connect(this, &ThreadedUndoRedoSystem::stateChanged, action, [action, this, prefix]() {
        QString text;
        QMetaObject::invokeMethod(m_undoRedoSystemWorker, "redoText", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(QString, text));
        action->setText(prefix.isEmpty() ? tr("Redo %1").arg(text) : prefix.arg(text));
        bool canRedo = false;
        QMetaObject::invokeMethod(m_undoRedoSystemWorker, "canRedo", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(bool, canRedo));
        action->setEnabled(canRedo);
    });
    action->setEnabled(canRedo);
    action->setShortcut(QKeySequence::Redo);

    connect(action, &QAction::triggered, this, &ThreadedUndoRedoSystem::redo);
    return action;
}

/*!
 * \brief Creates and returns an undo QAction with the specified \a parent and
 *\a prefix.
 */
QAction *ThreadedUndoRedoSystem::createUndoAction(QObject *parent, const QString &prefix) const
{
    QMutexLocker locker(&m_mutex);
    QAction *action = new QAction(parent);
    bool canUndo = false;

    QMetaObject::invokeMethod(m_undoRedoSystemWorker, "canUndo", Qt::BlockingQueuedConnection,
                              Q_RETURN_ARG(bool, canUndo));
    QString text;

    if (canUndo)
    {
        QMetaObject::invokeMethod(m_undoRedoSystemWorker, "undoText", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(QString, text));

        if (!text.isEmpty())
        {
            text = prefix.isEmpty() ? tr("Undo %1").arg(text) : prefix.arg(text);

            action->setText(text);
        }
    }

    connect(this, &ThreadedUndoRedoSystem::stateChanged, action, [action, this, prefix]() {
        QString text;
        QMetaObject::invokeMethod(m_undoRedoSystemWorker, "undoText", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(QString, text));
        action->setText(prefix.isEmpty() ? tr("Undo %1").arg(text) : prefix.arg(text));
        bool canUndo = false;
        QMetaObject::invokeMethod(m_undoRedoSystemWorker, "canUndo", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(bool, canUndo));
        action->setEnabled(canUndo);
    });
    action->setEnabled(canUndo);
    action->setShortcut(QKeySequence::Undo);
    connect(action, &QAction::triggered, this, &ThreadedUndoRedoSystem::undo);
    return action;
}

